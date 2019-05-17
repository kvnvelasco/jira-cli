#[macro_use]
extern crate lazy_static;

mod context;
mod git_bindings;
mod jira_bindings;
mod modules;
mod utils;

use inflector::cases::{kebabcase::to_kebab_case, screamingsnakecase::to_screaming_snake_case};

use crate::jira_bindings::{initialize_jira_client, initialize_jira_context};
use clap::{self, load_yaml, App};
use fs;
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
    let workspace = fs::Workspace::new(&working_directory)?.child_workspace(&WORKDIR.to_owned())?;

    match app.subcommand() {
        ("set", Some(subcontext)) => {
            modules::set_context::start(subcontext, &workspace)?;
        }
        ("fetch", Some(subcontext)) => {
            match subcontext.subcommand() {
                ("issues", _) => {
                    let client = initialize_jira_client(&workspace)?;
                    let context = initialize_jira_context(&workspace);
                    //                    match validate
                    //                    modules::fetch_context::fetch_and_cache_jira_issues(&client, &workspace, )?;
                }
                _ => {}
            }
        }
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
