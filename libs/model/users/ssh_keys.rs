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

use crate::model::users::users;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ssh_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ssh_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_user_uid(user_uid: Uuid) -> Select<Self> {
        Self::find().filter(Column::UserUid.eq(user_uid))
    }
    pub fn find_by_ssh_key(ssh_key: String) -> Select<Self> {
        Self::find().filter(Column::SshKey.eq(ssh_key))
    }
}

impl Model {
    pub async fn user(&self, db: &DatabaseConnection) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find_by_id(self.user_uid).one(db).await
    }
}
