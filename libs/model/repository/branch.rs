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

use crate::model::repository::commit;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "branch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub repository_uid: Uuid,
    pub head: String,
    pub head_uid: Uuid,
    pub default_branch: bool,
    pub start_point: bool,  // is head
    pub from: Option<Uuid>, // from commit
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn commits(
        &self,
        db: &DatabaseConnection,
        page: u64,
        size: u64,
    ) -> Result<Vec<commit::Model>, DbErr> {
        commit::Entity::find()
            .filter(commit::Column::BranchUid.eq(self.uid))
            .order_by_desc(commit::Column::Time)
            .paginate(db, size)
            .fetch_page(page)
            .await
    }
}
