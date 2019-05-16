use super::errors::{Error, Result};
use super::validate_context::{validate, RecommendedAction};
use crate::jira_bindings;
use crate::jira_bindings::{
    initialize_jira_client, initialize_jira_context, save_issues_to_cache, save_jira_context,
};
use core::borrow::BorrowMut;
use fs::Workspace;
use jira::{client::Client, Context};

pub fn fetch_and_cache_jira_issues(
    client: &Client,
    workspace: &Workspace,
    board_id: usize,
    sprint_id: usize,
) -> Result<()> {
    let issues = jira::issue::get_issues_for_sprint(&client, board_id, sprint_id, 0)?.issues;
    save_issues_to_cache(&workspace, &issues);
    Ok(())
}
