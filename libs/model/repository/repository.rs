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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_uid: Uuid,
    pub default_branch: String,
    pub visible: bool,
    pub template: bool,
    pub mirrors: bool,
    pub archive: bool,
    pub archive_time: Option<i64>,
    pub ssh_path: String,
    pub http_path: String,
    pub storage_node: String,
    pub fork: bool,
    pub fork_uid: Option<Uuid>,
    pub nums_star: i64,
    pub nums_fork: i64,
    pub nums_watch: i64,
    pub nums_issue: i64,
    pub nums_pull: i64,
    pub nums_commit: i64,
    pub head: String,
    pub license: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_owner(owner_uid: Uuid) -> Select<Entity> {
        Entity::find().filter(Column::OwnerUid.eq(owner_uid))
    }
    pub fn find_by_uid(uid: Uuid) -> Select<Entity> {
        Entity::find().filter(Column::Uid.eq(uid))
    }
}


impl ActiveModel {
    pub fn new(
        name: String,
        owner_uid: Uuid,
        description: Option<String>,
        visible: bool,
        default_branch: Option<String>,
    ) -> Self {
        Self {
            uid: Set(Uuid::new_v4()),
            name: Set(name.clone()),
            owner_uid: Set(owner_uid),
            description: Set(description),
            visible: Set(visible),
            default_branch: Set(default_branch.unwrap_or("".to_string())),
            template: Set(false),
            mirrors: Set(false),
            archive: Set(false),
            archive_time: Set(None),
            ssh_path: Set(format!("git@gitdata.ai:{}/{}", owner_uid, name)),
            http_path: Set(format!("https://gitdata.ai/{}/{}", owner_uid, name)),
            storage_node: Set("".to_string()),
            fork: Set(false),
            fork_uid: Set(None),
            nums_star: Set(0),
            nums_fork: Set(0),
            nums_watch: Set(0),
            nums_issue: Set(0),
            nums_pull: Set(0),
            nums_commit: Set(0),
            head: Set("".parse().unwrap()),
            license: Set(vec![]),
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
        }
    }
}