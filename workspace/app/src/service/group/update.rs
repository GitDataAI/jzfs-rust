use anyhow::anyhow;
use log::error;
use uuid::Uuid;
use dto::group::GroupUpdate;
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn update(&self, dto: GroupUpdate, uid: Uuid) -> anyhow::Result<i32>{
        let mut effect_row = 0;
        let model = GroupModel::select_by_column(&self.db, "uid", dto.group_id).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Group Not Found")
            })?
            .first()
            .map(|x|x.clone());
        
        if model.is_none(){
            return Err(anyhow!("[Error] Group Not Found"))
        }
        let mut model = model.unwrap();
        if self.check_group_owner(dto.group_id, uid).await?{
            return Err(anyhow!("[Error] You are not the owner of this group"))
        }
        if let Some(name) = dto.name{
            if self.check_no_name(name.clone()).await?{
                return Err(anyhow!("[Error] Group Name Already Exists"))
            }
            model.name = name;
            GroupModel::update_by_column(
                &self.db,
                &model,
                "uid"
            ).await
                .map_err(|x| {
                    error!("{}", x);
                    anyhow!("[Error] Update Group Failed")
                })?;
            effect_row += 1;
        }
        if let Some(bio) = dto.bio{
            model.bio = bio;
            GroupModel::update_by_column(
                &self.db,
                &model,
                "uid"
            )
                .await
                .map_err(|x| {
                    error!("{}", x);
                    anyhow!("[Error] Update Group Failed")
                })?;
            effect_row += 1;
        }
        if let Some(avatar) = dto.avatar{
            model.avatar = avatar;
            GroupModel::update_by_column(
                &self.db,
                &model,
                "uid"
            ).await
                .map_err(|x| {
                    error!("{}", x);
                    anyhow!("[Error] Update Group Failed")
                })?;
            effect_row += 1;
        };
        Ok(effect_row)
    }
}