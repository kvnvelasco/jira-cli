mod utils;

use clap::{self, load_yaml, App};
use jira::client::Client;
use jira::{self, Context};

use utils::io::{pick_from_list, read_line, Pickable};

impl Pickable for jira::issue::Issue {
    fn get_key(&self) -> String {
        self.key.to_owned()
    }
}

impl Pickable for jira::board::Board {
    fn get_key(&self) -> String {
        format!("{}", &self.id)
    }
}

impl Pickable for jira::sprint::Sprint {
    fn get_key(&self) -> String {
        format!("{}", &self.id)
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let _app = App::from_yaml(load_yaml!("cli_args.yml")).get_matches();

    let working_directory = std::env::current_dir()?;
    let workspace =
        fs::Workspace::new(&working_directory).child_workspace(&String::from("./.workflow"));
    workspace.create_directories()?;

    let jira_workspace = workspace.child_workspace(&fs::PathBuf::from("./jira"));
    jira_workspace.create_directories()?;
    jira_workspace.create_file(&fs::PathBuf::from("./session.yml"));
    jira_workspace.create_file(&fs::PathBuf::from("./context.yml"));

    let jira_session_file = jira_workspace.read_file(&fs::Path::new("./session.yml"))?;

    let client: Client = match serde_yaml::from_str(&jira_session_file) {
        Ok(client) => client,
        Err(_) => {
            println!("You don't have a jira session set up yet");
            let domain = read_line("Provide your atlassian domain (e.g: company.atlassian.net)");
            let email = read_line("Enter your email");
            println!(
                "Enter your atlassian API key. \
                 \nFor instructions on how to generate an api key, visit \
                 \nhttps://confluence.atlassian.com/cloud/api-tokens-938839638.html"
            );

            let api_key = read_line("Your API key");
            let client = Client::new(&domain, &api_key, &email);

            // save it
            let string = serde_yaml::to_string(&client)?;
            jira_workspace.write_file(fs::Path::new("./session.yml"), &string)?;

            client
        }
    };

    let jira_context_file = jira_workspace.read_file(&fs::Path::new("./context.yml"))?;

    let _context: Context = match serde_yaml::from_str(&jira_context_file) {
        Ok(context) => context,
        Err(_) => {
            println!("You don't have an active context set up yet");
            let boards = jira::board::get_boards(&client)?;
            let board_index = pick_from_list(
                "Select one project from the list Above",
                &boards.values,
                false,
            )?;
            let board = &boards.values[board_index];
            let sprints = board.get_sprints(&client)?;
            let sprint_index = pick_from_list(
                "Select one sprint from the list Above",
                &sprints.values,
                false,
            )?;
            let sprint = &sprints.values[sprint_index];

            let context = Context {
                active_board: board.id,
                active_sprint: sprint.id,
                active_issue: None,
            };

            // let issues = jira::issue::get_issues_for_sprint(
            //     &client,
            //     &context.active_sprint,
            //     &context.active_board,
            //     0,
            // )?;

            // let issue_index =
            //     pick_from_list("Select one from the list Above", &issues.issues, true)?;
            // let issue = &issues.issues[issue_index];

            let string = serde_yaml::to_string(&context)?;
            jira_workspace.write_file(fs::Path::new("./context.yml"), &string)?;

            context
        }
    };

    Ok(())
}
