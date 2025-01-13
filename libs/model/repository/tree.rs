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

#[derive(Debug, Clone, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tree")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub commit_uid: Uuid,
    pub branch_uid: Uuid,
    pub repository_uid: Uuid,
    pub hash: String,
    pub tree: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tree {
    pub filename: String,
    pub path: String,
    pub size: i64,
}

impl Model {
    pub fn tree(&self) -> Vec<Tree> {
        let mut result = vec![];
        for item in self.tree.iter() {
            if let Ok(tree) = serde_json::from_str::<Tree>(item) {
                result.push(tree);
            } else {
                continue;
            }
        }
        result
    }
    pub fn set_tree(&mut self, tree: Vec<Tree>) {
        self.tree = tree
            .iter()
            .map(|item| serde_json::to_string(item).unwrap())
            .collect();
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
