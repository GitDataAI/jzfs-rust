use rbatis::RBatis;


#[derive(Clone)]
pub struct AuthService {
    pub(crate) _db: RBatis,
}