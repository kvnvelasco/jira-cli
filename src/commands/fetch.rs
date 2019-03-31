use crate::jira::{get_boards, get_issues, Issue};
use crate::utils::config::Config;
use crate::utils::fs::{create_file};
use std::path::PathBuf;

pub fn boards(config: Config) {
  let _boards = get_boards(&config.session);
}

pub fn issues(config: Config) {
  let issues = match get_issues(&config, None) {
    Ok(issues) => issues.issues,
    Err(_) => {
      println!("Unable to fetch issues");
      std::process::exit(2);
    }
  };

  for issue in issues {
    let mut path = PathBuf::from(&config.paths.issues_dir);
    path.push(format!("{}.yml", issue.key));
    // attempt to fetch the issue if it exists
    match Issue::load(&path) {
      Some(_existing_issue) => {
        // detect changes
      }
      None => {
        create_file(&path);
        issue.persist(&path);
      },
    };
  }
}
