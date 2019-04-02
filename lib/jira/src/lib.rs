#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate ansi_term;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
  pub active_board: usize,
  pub active_issue: Option<usize>,
  pub active_sprint: usize,
}

pub mod board;
pub mod issue;
pub mod project;
pub mod sprint;
pub mod client;
pub mod common;
mod network;
mod urls;

