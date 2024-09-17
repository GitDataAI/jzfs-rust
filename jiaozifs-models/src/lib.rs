#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use tokio::sync::OnceCell;

pub static DB: OnceCell<sqlx::PgPool> = OnceCell::const_new();



pub mod aksks;
pub mod branches;
pub mod commits;
pub mod members;
pub mod merge_requests;
pub mod repositories;
pub mod tags;

pub mod trees;
mod filemode;
pub mod users;
pub mod wips;




pub type MatchMode = i64;

pub const ExactMatch:MatchMode = 0;
pub const PrefixMatch:MatchMode = 1;
pub const SuffixMatch:MatchMode = 2;
pub const LikeryMatch:MatchMode = 3;