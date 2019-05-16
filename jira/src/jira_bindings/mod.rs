use crate::utils::io::{read_line, Pickable};
use fs::Workspace;
use jira::issue::get_issue;
use jira::{client::Client, issue::Issue, Context};
use std::error::Error;

mod tests;

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

const ISSUE_CACHE_DIR: &'static str = "issue-cache";

pub fn initialize_jira_client(jira_workspace: &Workspace) -> Result<Client, Box<Error>> {
    let client = {
        let jira_session_file = jira_workspace.read_file(&fs::Path::new("./session.yml"))?;
        match serde_yaml::from_str::<Client>(&jira_session_file) {
            Ok(client) => client,
            Err(_) => {
                println!("You don't have a jira session set up yet");
                let domain =
                    read_line("Provide your Atlassian domain (e.g: company.atlassian.net)");
                let email = read_line("Enter your email");
                println!(
                    "Enter your atlassian API key. \
                     \nFor instructions on how to generate an api key, visit \
                     \nhttps://confluence.atlassian.com/cloud/api-tokens-938839638.html"
                );

                let api_key = read_line("Your API key");
                let client = Client::new(&domain, &api_key, &email);

                save_jira_client(&client, &jira_workspace).expect("Unable to save session file");

                client
            }
        }
    };
    Ok(client)
}

pub fn save_jira_client(client: &Client, jira_workspace: &Workspace) -> Result<(), Box<Error>> {
    jira_workspace.write_file(
        fs::Path::new("./session.yml"),
        &serde_yaml::to_string(&client)?,
    )?;
    Ok(())
}

pub fn initialize_jira_context(jira_workspace: &Workspace) -> Result<Context, Box<Error>> {
    let context = {
        let jira_context_file = jira_workspace.read_file(&fs::Path::new("./context.yml"))?;

        match serde_yaml::from_str::<Context>(&jira_context_file) {
            Ok(c) => c,
            Err(_) => {
                let context = Context {
                    active_issue: None,
                    active_sprint: None,
                    active_board: None,
                    issues_fetched: false,
                };
                save_jira_context(&context, &jira_workspace)?;
                context
            }
        }
    };
    Ok(context)
}

pub fn save_jira_context(context: &Context, jira_workspace: &Workspace) -> Result<(), Box<Error>> {
    jira_workspace.write_file(
        fs::Path::new("./context.yml"),
        &serde_yaml::to_string(&context)?,
    )?;
    Ok(())
}

pub fn load_issues_from_cache(
    workspace: &Workspace,
) -> Result<Vec<jira::issue::Issue>, Box<Error>> {
    let issue_files =
        {
            let cache_workspace_path = std::path::PathBuf::from(ISSUE_CACHE_DIR);
            workspace.get_files_for_dir(&cache_workspace_path).ok_or(
                simple_error::SimpleError::new("Unable to read issue files from cache"),
            )?
        };
    let issues: Vec<Issue> = issue_files
        .map(|file| {
            if let Ok(entry) = file {
                let string = {
                    let path_string = entry.path();
                    std::fs::read_to_string(&path_string).unwrap()
                };
                jira::issue::Issue::from_yml(string)
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect();
    Ok(issues)
}

pub fn save_issues_to_cache(workspace: &Workspace, issues: &Vec<jira::issue::Issue>) {
    for issue in issues {
        let path = {
            let child_workspace = workspace.child_workspace(&ISSUE_CACHE_DIR).unwrap();
            let mut path = child_workspace.workdir_path();
            path.push(format!("{}.yml", issue.key));
            path
        };
        if let Some(issue_string) = issue.to_yml() {
            workspace
                .write_file(&path, &issue_string)
                .expect("Unable to save file");
        }
    }
}

pub fn load_issue(workspace: &Workspace, issue_id: &str) -> Option<jira::issue::Issue> {
    //   Attempt to load issue from cache or fetch the real thing
    load_issue_from_cache(&workspace, &issue_id).or_else(|| {
        let client = initialize_jira_client(&workspace).ok()?;
        let context = initialize_jira_context(&workspace).ok()?;
        get_issue(&client, &issue_id)
    })
}

pub fn load_issue_from_cache(workspace: &Workspace, issue_id: &str) -> Option<jira::issue::Issue> {
    let child_workspace = workspace
        .child_workspace(&ISSUE_CACHE_DIR)
        .expect("Unable to load workspace");
    let mut path = child_workspace.workdir_path();
    path.push(format!("{}.yml", issue_id));
    if let Some(file_string) = child_workspace.read_file(&path).ok() {
        jira::issue::Issue::from_yml(file_string)
    } else {
        None
    }
}

pub fn clear_cache(workspace: &Workspace) {
    let cache_workspace_path = std::path::PathBuf::from(ISSUE_CACHE_DIR);
    let issue_files = workspace.get_files_for_dir(&cache_workspace_path);
    if let Some(files) = issue_files {
        for file in files {
            match file {
                Ok(entry) => {
                    let path_string = entry.path();
                    std::fs::remove_file(&path_string);
                }
                _ => {}
            }
        }
    };
}
