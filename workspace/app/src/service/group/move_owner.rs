use std::collections::HashMap;
use anyhow::anyhow;
use log::error;
use uuid::Uuid;
use dto::group::GroupOwnerMove;
use model::groups::group_users::GroupUserModel;
use model::groups::group_users_access::{GroupUsersAccess, GroupUsersAccessEnum};
use crate::service::group::GroupService;

impl GroupService {
    pub async fn move_owner(&self, dto: GroupOwnerMove, uid: Uuid) -> anyhow::Result<()>{
        if self.check_group_owner(dto.group_id, uid).await?{
            return Err(anyhow!("[Error] You are not the owner of this group"))
        }
        let group_users = GroupUserModel::select_by_column(&self.db, "group_id", dto.group_id).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Users Not Found")
            })?
            .first()
            .map(|x|x.clone());
        if group_users.is_none(){
            return Err(anyhow!("[Error] Group Users Not Found"))
        }
        let mut group_users = group_users.unwrap();
        let owner = group_users.users_id.0.iter().filter(|x|{
            x.1.access.clone() == GroupUsersAccessEnum::AccessOwner
        })
            .map(|x| (x.0.clone(),x.1.clone()))
            .collect::<HashMap<Uuid, GroupUsersAccess>>();
        if owner.contains_key(&dto.new_user_id){
            return Err(anyhow!("[Error] New Owner is already the owner of this group"))
        }
        group_users.users_id.0.insert(dto.new_user_id, owner.get(&uid).unwrap().clone());
        GroupUserModel::update_by_column(
            &self.db,
            &group_users,
            "uid"
        ).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Update Group Users Failed")
            })?;
        Ok(())
    }
}