mod context;
mod jira_bindings;
mod modules;
mod utils;
use clap::{self, load_yaml, App};
use fs;
use modules::get_context::{
    self,
    ContextTypes::{Git, JiraBacklog, JiraSprint},
};
// use jira_bindings::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let yml = load_yaml!("cli_args.yml");
    let app = App::from_yaml(yml).get_matches();

    let working_directory = std::env::current_dir()?;
    let workspace =
        fs::Workspace::new(&working_directory).child_workspace(&String::from("./.workflow"));

    workspace.create_directories()?;

    match app.subcommand() {
        ("fetch-context", Some(args)) => {
            let args = get_context::Arguments {
                source: match args.value_of("source") {
                    Some("jira-sprint") => JiraSprint,
                    Some("jira-backlog") => JiraBacklog,
                    Some("git") => Git,
                    _ => panic!("Invalid source"),
                },
            };

            match args.source {
                JiraBacklog | JiraSprint => {
                    let jira_workspace = workspace.child_workspace(&fs::PathBuf::from("./jira"));
                    get_context::run(&jira_workspace, args);
                }
                Git => {
                    let git_workspace = fs::Workspace::new(&working_directory);
                }
            };
        }
        ("on", Some(args)) => match args.value_of("context") {
            Some("jira") => {
                let reference = args.value_of("REF").expect("A resource name is required");
                git::create_branch_on_master(reference);
            }
            _ => {}
        },
        _ => {}
    };

    Ok(())
}
