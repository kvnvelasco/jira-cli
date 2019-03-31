use crate::jira::Issue;
use crate::utils::config::Config;
use std::fs::read_dir;
use std::io::Error;

pub struct Options {
  pub machine_ready: bool,
}

pub fn issues(config: &Config, options: Options) -> Result<(), Error> {
  let mut issues: Vec<Issue> = Vec::new();
  for entry in read_dir(&config.paths.issues_dir)? {
    let path = entry?.path();
    // try to materialize into issues
    if let Some(issue) = Issue::load(&path) {
      match options.machine_ready {
        true => println!(
          "{}-{}",
          issue.key,
          issue.fields.summary.to_lowercase().replace(" ", "-")
        ),
        false => println!("{}\t{}", issue.key, issue.fields.summary),
      }
      issues.push(issue);
    }
  }
  Ok(())
}
