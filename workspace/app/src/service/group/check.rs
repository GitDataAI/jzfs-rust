use std::collections::HashMap;
use anyhow::anyhow;
use log::error;
use uuid::Uuid;
use model::groups::group_users::GroupUserModel;
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn check_no_uuid(&self, uid: Uuid) -> anyhow::Result<bool>{
        let group = GroupModel::select_by_column(&self.db, "uid", uid).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Not Found")
            })?
            .first()
            .map(|x|x.clone());
        if group.is_none(){
            return Ok(true)
        }
        Ok(false)
    }
    pub async fn check_no_name(&self, name: String) -> anyhow::Result<bool>{
        let group = GroupModel::select_by_column(&self.db, "name", name).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Not Found")
            })?
            .first()
            .map(|x|x.clone());
        if group.is_none(){
            return Ok(true)
        }
        Ok(false)
    }
    pub async fn check_group_owner(&self, group_id: Uuid, uid: Uuid) -> anyhow::Result<bool>{
        let group = GroupUserModel::select_by_column(
            &self.db, 
            "group_id",
            group_id
        ).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Not Found")
            })?
            .first()
            .map(|x|x.clone());
        if group.is_none(){
            return Err(anyhow!("[Error] Group Not Found"))
        }
        let group = group.unwrap();
        let owner = group.users_id.0.iter().filter(|x|{
            x.1.access.clone() == model::groups::group_users_access::GroupUsersAccessEnum::AccessOwner
        })
            .map(|x| (x.0.clone(),x.1.clone()))
            .collect::<HashMap<Uuid, model::groups::group_users_access::GroupUsersAccess>>();
        if owner.contains_key(&uid){
            return Ok(true)
        }
        Ok(false)
    }
}