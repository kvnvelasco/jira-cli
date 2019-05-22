use super::errors::{Error, Result};
use crate::jira_bindings::{
    clear_cache, initialize_jira_client, initialize_jira_context, load_issues_from_cache,
    save_issues_to_cache, save_jira_context,
};

use crate::modules::fetch_context::fetch_and_cache_jira_issues;
use crate::modules::validate_context::{validate, RecommendedAction};
use crate::utils::io::pick_from_list;
use clap::ArgMatches;
use fs::Workspace;
use jira::{board, issue, sprint, Context};

pub fn start(arguments: &ArgMatches, workspace: &Workspace) -> Result<()> {
    let mut context = initialize_jira_context(&workspace)?;
    match arguments.subcommand() {
        ("issue", Some(_)) => {
            match validate(&context) {
                RecommendedAction::FetchIssues {
                    board_id,
                    sprint_id,
                } => {
                    let client = initialize_jira_client(&workspace)?;
                    fetch_and_cache_jira_issues(&client, &workspace, board_id, sprint_id)?;
                    context.issues_fetched = true;
                }
                RecommendedAction::FetchBoard => {
                    return Err(Box::new(Error::BoardContextUnavailable));
                }
                RecommendedAction::FetchSprint { .. } => {
                    return Err(Box::new(Error::SprintContextUnavailable));
                }
                RecommendedAction::SetIssue => {} // Do nothing, we're ok here
                _ => {
                    return Err(Box::new(Error::ContextUnparseable));
                }
            };
            set_issue(&workspace, &mut context)?;
            save_jira_context(&context, &workspace)?;
            Ok(())
        }
        ("sprint", Some(_)) => match validate(&context) {
            RecommendedAction::FetchBoard => {
                return Err(Box::new(Error::BoardContextUnavailable));
            }
            RecommendedAction::FetchSprint { board_id }
            | RecommendedAction::FetchAny { board_id, .. } => {
                let client = initialize_jira_client(&workspace)?;
                let sprints = sprint::get_sprint_for_board(&client, board_id)?;
                let selected = pick_from_list(&sprints.values)?;
                let sprint = &sprints.values[selected];
                context.active_sprint = Some(sprint.id);
                context.issues_fetched = false;
                save_jira_context(&context, &workspace)?;
                clear_cache(&workspace);
                println!("Selected Sprint. Active Sprint is now: {}", sprint);
                Ok(())
            }
            _ => return Err(Box::new(Error::ContextUnparseable)),
        },
        ("board", Some(_)) => {
            let client = initialize_jira_client(&workspace)?;
            let boards = board::get_boards(&client)?;
            let selected = pick_from_list(&boards.values)?;
            let board = &boards.values[selected];
            context.active_board = Some(board.id);
            context.active_sprint = None;
            context.issues_fetched = false;
            save_jira_context(&context, &workspace)?;
            clear_cache(&workspace);
            println!("Selected Board. Active Board is now: {}", board);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn set_issue(workspace: &Workspace, context: &mut Context) -> Result<()> {
    let issues = load_issues_from_cache(&workspace)?;
    let selected = pick_from_list(&issues)?;
    let issue = &issues[selected];
    context.active_issue = Some(issue.key.to_owned());
    Ok(())
}
