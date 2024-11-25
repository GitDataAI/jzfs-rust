use serde::{Deserialize, Serialize};
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub enum TeamUserAccess{
    AccessModeNone,
    AccessModeRead(Vec<Uuid>),
    AccessModeWrite(Vec<Uuid>),
    AccessModeAdmin(Vec<Uuid>),
}