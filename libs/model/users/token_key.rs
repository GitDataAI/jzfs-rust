/*
 *
 *  * Copyright (c) 2024-2025, GitDataAI Ltd.
 *  * All rights reserved.
 *  *
 *  * Licensed under your choice of the GitDataAI Source Available License 1.0
 *  * (Licensed_GSALv1) or the Server Side Public License v1 (Licensed_SSPLv1).
 *
 */
use chrono::Utc;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub user_uid: Uuid,
    pub token: String,
    pub access: i32, // unix access; 1 read 2 write 4 owner 7 all 0 none
    pub created_at: i64,
    pub updated_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new_token(
        name: String,
        description: Option<String>,
        user_uid: Uuid,
        token: String,
        access: i32,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            uid: Set(Uuid::new_v4()),
            name: Set(name),
            description: Set(description),
            user_uid: Set(user_uid),
            token: Set(token),
            access: Set(access),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}
