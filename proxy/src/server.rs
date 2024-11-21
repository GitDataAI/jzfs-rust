use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use pingora_core::prelude::HttpPeer;
use pingora_load_balancing::LoadBalancer;
use pingora_load_balancing::prelude::RoundRobin;
use pingora_proxy::{ProxyHttp, Session};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use cookie::Cookie;
use tracing::error;

pub(crate) type SessionState = HashMap<String, String>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Backend {
    Home,
    Cloud,
    Api,
}
#[derive(Clone)]
pub struct LB(pub Arc<HashMap<Backend, LoadBalancer<RoundRobin>>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = PooledConnection<RedisConnectionManager>;

    fn new_ctx(&self) -> Self::CTX {
        let manager = RedisConnectionManager::new("redis://221.128.225.26:6379").unwrap();
        let pool = Pool::builder()
            .build(manager)
            .unwrap();
        pool.get().unwrap()
    }

    async fn upstream_peer(&self, session: &mut Session, ctx: &mut Self::CTX) -> pingora::Result<Box<HttpPeer>> {
        let req = session.req_header().uri.clone();
        let mut peer = if req.to_string().starts_with("/api") {
            self.0.get(&Backend::Api).unwrap()
        } else {
            self.0.get(&Backend::Home).unwrap()
        };
        if let Some(cookies) = session.get_header("cookie") {
            if !cookies.is_empty() {
                for cookie_str in cookies.to_str().unwrap().split(";") {
                    let cookie = Cookie::parse(cookie_str).unwrap();
                    if cookie.name() == "SessionID" {
                        let session_id = cookie.value().to_string();
                        let session_data = ctx.get::<_, String>(session_id);
                        match session_data {
                            Ok(data) => {
                                let session_state: SessionState = serde_json::from_str(&data).unwrap();
                                if session_state.get("IsLogin").is_some() {
                                    peer = self.0.get(&Backend::Cloud).unwrap();
                                    break;
                                }
                            }
                            Err(e) => {error!("{}",e.to_string())}
                        }
                        break;
                    }
                }
            }
        }
        Ok(Box::new(HttpPeer::new(peer.select(b"",256).unwrap(), true, "".to_string())))
    }
}