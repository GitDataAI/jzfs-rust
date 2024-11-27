use std::collections::HashMap;
use anyhow::anyhow;
use log::error;
use rbatis::rbdc::JsonV;
use uuid::Uuid;
use dto::group::GroupCreate;
use model::groups::group_users::GroupUserModel;
use model::groups::group_users_access::{GroupUsersAccess, GroupUsersAccessEnum};
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn create(&self, dto: GroupCreate, uid: Uuid) -> anyhow::Result<()>{
        let uuid = {
            loop {
                let uid = Uuid::new_v4();
                if self.check_no_uuid(uid).await?{
                    break uid;
                }
            }
        };
        if !self.check_no_name(dto.name.clone()).await?{
            return Err(anyhow!("[Error] Group Name Already Exists"))
        }
        let group_model = GroupModel{
            uid: uuid,
            name: dto.name.clone(),
            bio: dto.bio,
            avatar: dto.avatar,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        GroupModel::insert(
            &self.db,
            &group_model
        ).await.map_err(|e| {
            error!("{}", e);
            anyhow!("[Error] Create Group Failed")
        })?;
        let group_users = GroupUserModel{
            uid: Uuid::new_v4(),
            group_id: uuid,
            users_id: JsonV(
                HashMap::from([
                    {
                        (uid, GroupUsersAccess{
                            access: GroupUsersAccessEnum::AccessOwner,
                            join_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                        })
                    }
                ])
            ),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
       
        GroupUserModel::insert(
            &self.db,
            &group_users
        ).await.map_err(|e|{
            error!("{}", e);
            anyhow!("[Error] Create Group Failed")
        })?;
        Ok(())
    }
}