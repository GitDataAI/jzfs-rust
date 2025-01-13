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
    pub archive_time: Option<chrono::NaiveDateTime>,
    pub ssh_path: String,
    pub http_path: String,
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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
