mod utils;
mod jira_bindings;
mod context;
use clap::{self, load_yaml, App};

use jira_bindings::Initialisable;

fn main() -> Result<(), Box<std::error::Error>> {
    let _app = App::from_yaml(load_yaml!("cli_args.yml")).get_matches();

    let working_directory = std::env::current_dir()?;
    let workspace =
        fs::Workspace::new(&working_directory).child_workspace(&String::from("./.workflow"));
    workspace.create_directories()?;

    let jira_workspace = workspace.child_workspace(&fs::PathBuf::from("./jira"));

    Ok(())
}
