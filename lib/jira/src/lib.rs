use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
    pub active_board: usize,
    pub active_issue: Option<String>,
    pub active_sprint: usize,
}

pub mod board;
pub mod client;
pub mod common;
pub mod issue;
mod network;
pub mod project;
pub mod sprint;
mod urls;
