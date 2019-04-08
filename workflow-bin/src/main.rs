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

    let jira_workspace = workspace.child_workspace(&fs::PathBuf::from("./jira"));

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
            if JiraBacklog == args.source || JiraSprint == args.source {
                get_context::run(&jira_workspace, args);
            };
        }
        _ => {}
    };

    Ok(())
}
