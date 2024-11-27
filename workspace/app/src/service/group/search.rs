use anyhow::anyhow;
use log::error;
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn search_by_name(&self, name: String) -> anyhow::Result<Vec<GroupModel>>{
        let models = GroupModel::select_by_name(
            &self.db,
            name
        )
            .await
            .map_err(|e|{
                error!("{}", e);
                anyhow!("[Error] Search Group Failed")
            })?;
        Ok(models)
    }
}