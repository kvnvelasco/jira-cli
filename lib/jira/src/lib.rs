use serde_derive::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Context {
    pub active_board: Option<usize>,
    pub active_issue: Option<String>,
    pub active_sprint: Option<usize>,
    pub issues_fetched: bool,
}

pub mod board;
pub mod client;
pub mod common;
pub mod issue;
mod network;
pub mod project;
pub mod sprint;
mod urls;
