/*
 *
 *  * Copyright (c) 2024-2025, GitDataAI Ltd.
 *  * All rights reserved.
 *  *
 *  * Licensed under your choice of the GitDataAI Source Available License 1.0
 *  * (Licensed_GSALv1) or the Server Side Public License v1 (Licensed_SSPLv1).
 *
 */

use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "commits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub branch_uid: Uuid,
    pub repository_uid: Uuid,
    pub head: String,
    pub msg: String,
    pub time: i64,
    pub user: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub parents: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn parents(&self, db: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
        if let Some(parents) = self.parents {
            let parent = Entity::find_by_id(parents).one(db).await?;
            return Ok(parent);
        }
        Ok(None)
    }
    pub async fn children(&self, db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        let children = Entity::find()
            .filter(Column::Parents.eq(self.uid))
            .all(db)
            .await?;
        Ok(children)
    }
}
