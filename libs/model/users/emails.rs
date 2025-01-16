/*
 *
 *  * Copyright (c) 2024-2025, GitDataAI Ltd.
 *  * All rights reserved.
 *  *
 *  * Licensed under your choice of the GitDataAI Source Available License 1.0
 *  * (GSALv1) or the Server Side Public License v1 (SSPLv1).
 *
 */
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "emails")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
