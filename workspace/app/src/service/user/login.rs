use dto::session::SessionUserValue;
use dto::users::{UserLogin, UserLoginEmail};
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn login_email(&self, dto: UserLoginEmail) -> anyhow::Result<SessionUserValue>{
        let info = UsersModel::select_by_column(
            &self.db,
            "email",
            dto.email
        ).await
            .map_err(|_| anyhow::anyhow!("[Error] Email Not Found"))?
            .first()
            .map(|x| x.clone());
        if info.is_none(){
            return Err(anyhow::anyhow!("[Error] Email Not Found"));
        }
        let info = info.unwrap();
        if info.passwd == dto.password{
            return Ok(SessionUserValue{
                uid: rbatis::rbdc::Uuid(info.uid.to_string()),
                name: info.name,
                pro: info.pro,
                username: info.username,
                email: info.email,
                phone: info.phone.expect("REASON"),
                bio: info.bio.expect("REASON"),
            })
        }
        Err(anyhow::anyhow!("[Error] Password Error"))
    }
    pub async fn login_name(&self, dto: UserLogin) -> anyhow::Result<SessionUserValue> {
        let info = UsersModel::select_by_column(
            &self.db,
            "username",
            dto.username
        ).await
            .map_err(|_| anyhow::anyhow!("[Error] UserName Not Found"))?
            .first()
            .map(|x| x.clone());
        if info.is_none(){
            return Err(anyhow::anyhow!("[Error] UserName Not Found"));
        }
        let info = info.unwrap();
        if info.passwd == dto.password{
            return Ok(SessionUserValue{
                uid: info.uid,
                name: info.name,
                pro: info.pro,
                username: info.username,
                email: info.email,
                phone: info.phone.expect("REASON"),
                bio: info.bio.expect("REASON"),
            })
        }
        Err(anyhow::anyhow!("[Error] Password Error"))
    }
}