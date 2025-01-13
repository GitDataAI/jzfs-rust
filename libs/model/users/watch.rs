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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "watch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub repository_uid: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
