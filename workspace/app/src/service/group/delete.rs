use std::collections::HashMap;
use anyhow::anyhow;
use log::error;
use rbatis::rbdc::Uuid;
use dto::group::GroupDelete;
use model::groups::group_users::GroupUserModel;
use model::groups::group_users_access::{GroupUsersAccess, GroupUsersAccessEnum};
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn delete(&self, dto: GroupDelete, uid: Uuid) -> anyhow::Result<()>{
        let group_id = dto.uid;
        let group = GroupModel::select_by_column(&self.db, "uid", group_id).await
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
        let group_users = GroupUserModel::select_by_column(
            &self.db, "group_id", group.uid
        ).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Users Not Found")
            })?
            .first()
            .map(|x|x.clone());
        if group_users.is_none(){
            return Err(anyhow!("[Error] Group Users Not Found"))
        }
        let group_users = group_users.unwrap();
        let owner = group_users.users_id.0.iter().filter(|x|{
            x.1.access.clone() == GroupUsersAccessEnum::AccessOwner
        })
            .map(|x| (x.0.clone(),x.1.clone()))
            .collect::<HashMap<Uuid, GroupUsersAccess>>();
        if owner.contains_key(&uid){
            GroupModel::delete_by_column(&self.db, "uid", group_id).await
                .map_err(|x| {
                    error!("{}", x);
                    anyhow!("[Error] Delete Group Failed")
                })?;
            GroupUserModel::delete_by_column(&self.db, "group_id", group_id).await
                .map_err(|x| {
                    error!("{}", x);
                    anyhow!("[Error] Delete Group Users Failed")
                })?;
        }else { 
            return Err(anyhow!("[Error] You are not the owner of this group"))
        }
        Ok(())
    }
}