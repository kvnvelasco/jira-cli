use crate::jira_bindings::{initialize_jira_client, initialize_jira_context, save_jira_context};
use crate::utils::io::pick_from_list;
use clap::ArgMatches;
use fs::Workspace;
use jira::{board, issue, sprint};

pub fn start(arguments: &ArgMatches, workspace: &Workspace) -> Result<(), Box<std::error::Error>> {
    match arguments.subcommand() {
        ("issue", Some(_)) => {
            let client = initialize_jira_client(&workspace)?;
            let mut context = initialize_jira_context(&workspace)?;
            if context.active_board == None {
                println!(
          "You don't have an active project set. \nRun jira set board before setting an issue"
        );
                return Ok(());
            };
            if context.active_sprint == None {
                println!(
          "You don't have an active sprint set. \nRun jira set sprint before setting an issue"
        );
                return Ok(());
            };
            let issues = issue::get_issues_for_sprint(
                &client,
                context.active_board.unwrap(),
                context.active_sprint.unwrap(),
                0,
            )?;
            let selected = pick_from_list(&issues.issues)?;
            let issue = &issues.issues[selected];
            context.active_issue = Some(issue.id.to_owned());
            save_jira_context(&context, &workspace)?;
        }
        ("sprint", Some(_)) => {
            let client = initialize_jira_client(&workspace)?;

            // let's get a context going
            let mut context = initialize_jira_context(&workspace)?;
            if context.active_board == None {
                println!(
          "You don't have an active project set. \nRun jira set board before setting an issue"
        );
                return Ok(());
            };
            let sprints = sprint::get_sprint_for_board(&client, context.active_board.unwrap())?;
            let selected = pick_from_list(&sprints.values)?;
            let sprint = &sprints.values[selected];
            context.active_sprint = Some(sprint.id);
            save_jira_context(&context, &workspace)?;
            println!("Selected Sprint. Active Sprint is now: {}", sprint)
        }
        ("board", Some(_)) => {
            let client = initialize_jira_client(&workspace)?;
            let mut context = initialize_jira_context(&workspace)?;
            let boards = board::get_boards(&client)?;
            let selected = pick_from_list(&boards.values)?;
            let board = &boards.values[selected];
            context.active_board = Some(board.id);
            save_jira_context(&context, &workspace)?;
            println!("Selected Board. Active Board is now: {}", board)
        }
        _ => {}
    };
    Ok(())
}
