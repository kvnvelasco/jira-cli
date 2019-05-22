use super::errors::{Error, Result};
use jira::Context;

pub enum RecommendedAction {
    FetchBoard,
    FetchSprint {
        board_id: usize,
    },
    FetchIssues {
        board_id: usize,
        sprint_id: usize,
    },
    SetIssue,
    FetchAny {
        board_id: usize,
        sprint_id: usize,
        issue_id: String,
    },
    NoAction,
}

pub fn validate(context: &Context) -> RecommendedAction {
    match context {
        Context {
            active_board: None,
            active_sprint: None,
            ..
        } => RecommendedAction::FetchBoard,
        Context {
            active_board: Some(active_board),
            active_sprint: None,
            ..
        } => RecommendedAction::FetchSprint {
            board_id: active_board.to_owned(),
        },
        Context {
            active_sprint: Some(active_sprint),
            active_board: Some(active_board),
            issues_fetched: false,
            ..
        } => RecommendedAction::FetchIssues {
            board_id: active_board.to_owned(),
            sprint_id: active_sprint.to_owned(),
        },
        Context {
            active_issue: None,
            issues_fetched: true,
            ..
        } => RecommendedAction::SetIssue,
        Context {
            active_sprint: Some(sprint_id),
            active_board: Some(board_id),
            active_issue: Some(issue_id),
            ..
        } => RecommendedAction::FetchAny {
            board_id: board_id.to_owned(),
            sprint_id: sprint_id.to_owned(),
            issue_id: issue_id.to_owned(),
        },
        _ => RecommendedAction::NoAction,
    }
}
