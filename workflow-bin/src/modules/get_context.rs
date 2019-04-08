use crate::jira_bindings::*;
use fs::Workspace;
use jira::{client::Client, Context};

#[derive(PartialEq, Debug)]
pub enum ContextTypes {
  JiraSprint,
  JiraBacklog,
  Git,
}

#[derive(Debug)]
pub struct Arguments {
  pub source: ContextTypes,
}

pub fn run(workspace: &Workspace, arguments: Arguments) {
  // TODO: handle this panic
  let client = Client::initialize(&workspace).unwrap();
  let context = Context::initialize(&workspace).unwrap();
  let mut issues: Vec<jira::issue::Issue> = Vec::new();
  match arguments.source {
    ContextTypes::JiraBacklog => {}
    ContextTypes::JiraSprint => {
      let issue_context =
        jira::issue::get_issues_for_sprint(&client, context.active_board, context.active_sprint, 0)
          .expect("Unable do get issues for sprint");
      for issue in issue_context.issues {
        issues.push(issue)
      }
    }
    ContextTypes::Git => {}
  };

  // persist all the issues
  for issue in &issues {
    let file_name = &issue.key;
    let file_string = serde_yaml::to_string(&issue).expect("Unable to serialize issue");

    let issue_workspace = workspace.child_workspace(&std::path::PathBuf::from("./issues"));
    issue_workspace.create_directories();
    issue_workspace
      .write_file(&std::path::PathBuf::from(file_name), &file_string)
      .unwrap();
  }
}
