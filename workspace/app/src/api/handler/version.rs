use std::env::consts::OS;
use std::time::Instant;
use actix_web::{web, Responder};

pub struct Version{
    pub start: String,
    pub version: String,
    pub os: String,
    pub pid: String,
    pub req: i64,
    pub cpu: String,
    pub mem: String,
    pub disk: String,
}


pub async fn api_version(time: web::Data<Instant>) -> impl Responder{
    format!("服务启动时长: {:?}\n版本: {}\n操作系统: {}\n进程ID: {}", time.elapsed(), "0.1", OS.to_string(), std::process::id())
}