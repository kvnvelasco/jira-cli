// Initializes traits for workspaces and clients

// use crate::context::{Context as WorkflowContext, ContextProvider};
use crate::utils::io::{pick_from_list, read_line, Pickable};
use fs::Workspace;
use jira::{client::Client, Context};
use std::error::Error;

impl Pickable for jira::issue::Issue {
    fn get_key(&self) -> String {
        self.key.to_owned()
    }
    fn display_key(&self) -> String {
        String::from("") // we don't display the key in this case because it's part of the display trait implemented on the issue
    }
}

impl Pickable for jira::board::Board {
    fn get_key(&self) -> String {
        format!("{}", &self.id)
    }
    fn display_key(&self) -> String {
        format!("{}", &self.id)
    }
}

impl Pickable for jira::sprint::Sprint {
    fn get_key(&self) -> String {
        format!("{}", &self.id)
    }
    fn display_key(&self) -> String {
        format!("{}", &self.id)
    }
}

pub trait Initializable: Sized {
    fn is_initialized(&self) -> bool;
    fn initialize(workspace: &Workspace) -> Result<Self, Box<Error>>;
}

impl Initializable for Client {
    fn is_initialized(&self) -> bool {
        true
    }
    fn initialize(jira_workspace: &Workspace) -> Result<Client, Box<Error>> {
        jira_workspace.create_file(&fs::PathBuf::from("./session.yml"));

        let jira_session_file = jira_workspace.read_file(&fs::Path::new("./session.yml"))?;

        Ok(match serde_yaml::from_str(&jira_session_file) {
            Ok(client) => client,
            Err(_) => {
                println!("You don't have a jira session set up yet");
                let domain =
                    read_line("Provide your atlassian domain (e.g: company.atlassian.net)");
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
        })
    }
}

impl Initializable for Context {
    fn is_initialized(&self) -> bool {
        true // this is always initialized because it must exist
    }
    fn initialize(jira_workspace: &Workspace) -> Result<Context, Box<Error>> {
        jira_workspace.create_file(&fs::PathBuf::from("./context.yml"));
        let jira_context_file = jira_workspace.read_file(&fs::Path::new("./context.yml"))?;
        let client = Client::initialize(&jira_workspace)?;
        Ok(match serde_yaml::from_str(&jira_context_file) {
            Ok(context) => context,
            Err(_) => {
                println!("You don't have an active context set up yet");
                let boards = jira::board::get_boards(&client)?;
                let board_index =
                    pick_from_list("Select one project from the list Above", &boards.values)?;
                let board = &boards.values[board_index];
                let sprints = board.get_sprints(&client)?;
                let sprint_index =
                    pick_from_list("Select one sprint from the list Above", &sprints.values)?;
                let sprint = &sprints.values[sprint_index];

                let context = Context {
                    active_board: board.id,
                    active_sprint: sprint.id,
                    active_issue: None,
                };

                let string = serde_yaml::to_string(&context)?;
                jira_workspace.write_file(fs::Path::new("./context.yml"), &string)?;

                context
            }
        })
    }
}
