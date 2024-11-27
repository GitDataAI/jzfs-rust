use anyhow::anyhow;
use log::error;
use rbatis::rbdc::Uuid;
use model::groups::group_users::GroupUserModel;
use model::groups::groups::GroupModel;
use crate::service::group::GroupService;

impl GroupService {
    pub async fn list_by_uid(&self, uid: Uuid) -> anyhow::Result<Vec<GroupModel>>{
        let models = GroupUserModel::select_all(
            &self.db
        ).await
            .map_err(|e|{
                error!("{}", e);
                anyhow!("[Error] List Group Users Failed")
            })?
            .iter()
            .filter(|x| {
                x.users_id
                    .0
                    .contains_key(&uid)
            })
            .map(|x| x.group_id.clone())
            .collect::<Vec<Uuid>>();
        let mut results = Vec::new();
        for model in models {
            let model = GroupModel::select_by_column(&self.db, "uid", model).await
                .map_err(|e|{
                    error!("{}", e);
                    anyhow!("[Error] List Group Users Failed")
                })?
                .first()
                .map(|x|x.clone());
            if model.is_none(){
                continue;
            }
            results.push(model.unwrap());
        }
        Ok(results)
    }
}