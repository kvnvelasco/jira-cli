#[macro_use]
extern crate lazy_static;

mod context;
mod git_bindings;
mod jira_bindings;
mod modules;
mod utils;

use inflector::cases::{kebabcase::to_kebab_case, screamingsnakecase::to_screaming_snake_case};

use crate::jira_bindings::{initialize_jira_client, initialize_jira_context};
use clap::Shell;
use clap::{self, load_yaml, App};
use fs;
use fs::Workspace;
use std::io::{BufWriter, Write};
use utils::io::confirm;

const WORKDIR: &'static str = "./.jira";

fn main() {
    match start() {
        Ok(()) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn start() -> Result<(), Box<std::error::Error>> {
    let yml = load_yaml!("cli_args.yml");
    let app = App::from_yaml(yml).get_matches();

    let working_directory = std::env::current_dir()?;
    let maybe_workspace = fs::Workspace::discover(&WORKDIR.to_owned());

    if let ("init", Some(_)) = app.subcommand() {
        match maybe_workspace {
            Some(workspace) => {
                println!("There is already an active jira context set up in this directory");
                if confirm("Would you like overwrite it") {
                    workspace.reset();
                    initialize_jira_client(&workspace);
                };
            }
            _ => {
                println!("Creating jira directory in this folder");
                Workspace::new(&working_directory)?.child_workspace(&WORKDIR.to_owned())?;
            }
        };
    }

    let workspace = fs::Workspace::discover(&WORKDIR.to_owned())
        .expect("You do not have a workspace set up in this directory");

    match app.subcommand() {
        ("completions", Some(subcontext)) => {
            if subcontext.is_present("SHELL") {
                let arg = subcontext
                    .value_of("SHELL")
                    .expect("No value provided for shell");
                let shell = arg.parse::<Shell>()?;
                let mut main_app = App::from_yaml(yml);
                let completions =
                    main_app.gen_completions_to("jira", Shell::Zsh, &mut std::io::stdout());
            };
        }
        ("set", Some(subcontext)) => {
            modules::set_context::start(subcontext, &workspace)?;
        }
        ("fetch", Some(subcontext)) => match subcontext.subcommand() {
            ("issues", _) => {
                let client = initialize_jira_client(&workspace)?;
                let context = initialize_jira_context(&workspace)?;
                match modules::validate_context::validate(&context) {
                    modules::validate_context::RecommendedAction::FetchIssues {
                        board_id,
                        sprint_id,
                    }
                    | modules::validate_context::RecommendedAction::FetchAny {
                        board_id,
                        sprint_id,
                        ..
                    } => {
                        modules::fetch_context::fetch_and_cache_jira_issues(
                            &client, &workspace, board_id, sprint_id,
                        )?;
                    }
                    modules::validate_context::RecommendedAction::FetchSprint { .. } => {
                        println!("You don't have a sprint context set up. Run jira set sprint")
                    }
                    modules::validate_context::RecommendedAction::FetchBoard { .. } => {
                        println!("You don't have a board context set up. Run jira set board")
                    }
                    _ => {}
                };
            }
            _ => {}
        },
        ("checkout", Some(_)) => {
            let context = jira_bindings::initialize_jira_context(&workspace)?;
            if let Some(issue_key) = context.active_issue {
                let mut repo = git_bindings::Repository::new(&working_directory)?;
                let key = {
                    let issue = jira_bindings::load_issue(&workspace, &issue_key).unwrap();
                    &format!(
                        "{}-{}",
                        to_screaming_snake_case(&issue.key).replace("_", "-"),
                        to_kebab_case(&issue.fields.summary)
                    )
                };
                repo.set_remote("origin");
                if repo.branch_exists(&key) {
                    if confirm("Checkout branch. This will reset your working tree") {
                        repo.set_branch(&key)?;
                        println!("Checked out branch {}", key);
                    };
                } else if confirm(&format!("Branch {} does not exist, create", key)) {
                    if repo.create_branch(&key).is_ok() {
                        println!("Created branch {}", &key);
                        if confirm("Checkout branch. This will reset your working tree") {
                            repo.set_branch(&key)?;
                        };
                    };
                }
            }
        }
        _ => {}
    };

    Ok(())
}
