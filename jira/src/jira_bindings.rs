use crate::utils::io::{read_line, Pickable};
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

pub fn initialize_jira_client(jira_workspace: &Workspace) -> Result<Client, Box<Error>> {
    let jira_session_file = jira_workspace.read_file(&fs::Path::new("./session.yml"))?;

    Ok(match serde_yaml::from_str(&jira_session_file) {
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

            save_jira_client(&client, &jira_workspace)?;

            client
        }
    })
}

pub fn save_jira_client(client: &Client, jira_workspace: &Workspace) -> Result<(), Box<Error>> {
    jira_workspace.write_file(
        fs::Path::new("./session.yml"),
        &serde_yaml::to_string(&client)?,
    )?;
    Ok(())
}

pub fn initialize_jira_context(jira_workspace: &Workspace) -> Result<Context, Box<Error>> {
    let jira_context_file = jira_workspace.read_file(&fs::Path::new("./context.yml"))?;
    Ok(match serde_yaml::from_str(&jira_context_file) {
        Ok(context) => context,
        Err(_) => {
            let context = Context {
                active_issue: None,
                active_sprint: None,
                active_board: None,
            };

            save_jira_context(&context, &jira_workspace)?;
            context
        }
    })
}

pub fn save_jira_context(context: &Context, jira_workspace: &Workspace) -> Result<(), Box<Error>> {
    jira_workspace.write_file(
        fs::Path::new("./context.yml"),
        &serde_yaml::to_string(&context)?,
    )?;
    Ok(())
}
