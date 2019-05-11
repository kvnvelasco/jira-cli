#[macro_use]
extern crate lazy_static;

mod context;
mod git_bindings;
mod jira_bindings;
mod modules;
mod utils;

use inflector::cases::{kebabcase::to_kebab_case, screamingsnakecase::to_screaming_snake_case};

use clap::{self, load_yaml, App};
use fs;
use jira::{board, issue, sprint};
use jira_bindings::{initialize_jira_client, initialize_jira_context, save_jira_context};
use utils::io::{confirm, pick_from_list};

const WORKDIR: &'static str = "./.jira";

fn main() {
    match start() {
        Ok(()) => {}
        Err(err) => {
            println!("{:?}", err);
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
        ("checkout", Some(_)) => {
            let mut repo = git_bindings::Repository::new(&working_directory)?;
            let client = jira_bindings::initialize_jira_client(&workspace)?;
            let context = jira_bindings::initialize_jira_context(&workspace)?;
            if let Some(issue_key) = context.active_issue {
                let issue = jira::issue::get_issue(&client, &issue_key).unwrap();
                let key = &format!(
                    "{}-{}",
                    to_screaming_snake_case(&issue.key).replace("_", "-"),
                    to_kebab_case(&issue.fields.summary)
                );

                repo.set_remote("origin");

                if repo.branch_exists(&key) {
                    if confirm("Branch already exists, checkout") {
                        repo.set_branch(&key);
                        println!("Checked out branch {}", key);
                    }
                } else {
                    if confirm(&format!("Branch {} does not exist, create", key)) {
                        //                        println!("Creating branch {} on {}", key, repo.ref_spec);
                        //                        println!("Fetching {}...", repo.ref_spec);
                        //                        repo.fetch_branch(&repo.ref_spec.clone());
                        repo.create_branch(&key);
                        println!("Created branch {}", &key);
                        if confirm("Checkout branch") { repo.set_branch(&key); }
                    }
                }
            }
        }
        _ => {}
    };

    Ok(())
}
