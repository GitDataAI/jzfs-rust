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

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub username: String,
    pub name: String,
    pub main_email: String,
    pub email_visible: bool,
    pub hash_pass: String,

    pub mindset: Option<String>,
    pub state: String, // active, blocked, deactivated, etc.
    pub avatar_url: Option<String>,

    pub company: Option<String>,
    pub job_title: Option<String>,
    pub website: Option<String>,
    pub social: Vec<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub appellative: Option<String>,
    pub topic: Vec<String>,
    pub pinned: Vec<Uuid>,

    pub repository_limit: i32, // default 20

    pub created_at: i64,
    pub updated_at: i64,
    pub last_used: Option<i64>,

    pub professional: bool,
    pub professional_end_time: Option<i64>,
    pub professional_start_time: Option<i64>,

    pub organize: bool,
    pub member: Vec<Uuid>,
    pub team: Vec<Uuid>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_username(username: &str) -> Select<Self> {
        Self::find().filter(Column::Username.eq(username))
    }
    pub fn find_by_uid(uid: Uuid) -> Select<Self> {
        Self::find().filter(Column::Uid.eq(uid))
    }
    pub fn find_by_email(email: &str) -> Select<Self> {
        Self::find().filter(Column::MainEmail.eq(email))
    }

    pub fn find_by_state(state: &str) -> Select<Self> {
        Self::find().filter(Column::State.eq(state))
    }
}

impl ActiveModel {
    fn defaults() -> Self {
        ActiveModel {
            uid: Set(Uuid::new_v4()),
            username: Set("".to_string()),
            name: Set("".to_string()),
            main_email: Set("".to_string()),
            email_visible: Set(false),
            hash_pass: Set("".to_string()),
            mindset: Set(None),
            state: Set("Active".to_string()),
            avatar_url: Set(None),
            company: Set(None),
            job_title: Set(None),
            website: Set(None),
            social: Set(vec![]),
            bio: Set(None),
            location: Set(None),
            appellative: Set(None),
            topic: Set(vec![]),
            pinned: Set(vec![]),
            repository_limit: Set(20),
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            last_used: Set(None),
            professional: Set(false),
            professional_end_time: Set(None),
            professional_start_time: Set(None),
            organize: Set(false),
            member: Set(vec![]),
            team: Set(vec![]),
        }
    }
    pub fn new_users(username: String, email: String, passwd: String) -> Self {
        let mut default = ActiveModel::defaults();
        default.uid = Set(Uuid::new_v4());
        default.username = Set(username.clone());
        default.main_email = Set(email.clone());
        default.hash_pass = Set(passwd);
        default.name = Set(username);
        default.main_email = Set(email);
        default
    }
    pub fn new_organize(name: String, email: String, member_uid: Uuid) -> ActiveModel {
        let mut default = ActiveModel::defaults();
        default.uid = Set(Uuid::new_v4());
        default.username = Set(name.clone());
        default.main_email = Set(email.clone());
        default.name = Set(name);
        default.main_email = Set(email);
        default.organize = Set(true);
        default.member = Set(vec![member_uid]);
        default
    }
}
