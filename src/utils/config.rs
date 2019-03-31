use super::fs::{construct_paths, file_to_string, write_file};
use serde_yaml;
use std::path::{Path, PathBuf};

pub struct Config {
  pub paths: Paths,
  pub session: Session,
  pub context: Context,
}

pub struct Paths {
  pub project: PathBuf,
  pub config_dir: PathBuf,
  pub config_file: PathBuf,
  pub context_file: PathBuf,
  pub session_file: PathBuf,
  pub issues_dir: PathBuf,
  pub boards_dir: PathBuf,
}

impl Paths {
  pub fn new(project_path: &Path, validate: bool) -> Paths {
    Paths {
      project: construct_paths(&project_path, vec![], validate),
      config_dir: construct_paths(&project_path, vec![".jira/"], validate),
      config_file: construct_paths(&project_path, vec![".jira/", "config.yml"], validate),
      context_file: construct_paths(&project_path, vec![".jira/", "context.yml"], validate),
      session_file: construct_paths(&project_path, vec![".jira/", "session.yml"], validate),
      issues_dir: construct_paths(&project_path, vec![".jira/", "issues/"], validate),
      boards_dir: construct_paths(&project_path, vec![".jira/", "boards/"], validate),
    }
  }
}

impl Config {
  pub fn load(project_path: &str) -> Config {
    let paths = Paths::new(Path::new(project_path), true);

    // validate paths 
  
    Config {
      session: Session::load(&paths.session_file),
      context: Context::load(&paths.context_file),
      paths: paths,
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
  pub active_board: Option<String>,
  pub active_issue: Option<String>,
  pub active_project: Option<String>,
  pub active_sprint: Option<String>,
}

impl Context {
  pub fn load(file_path: &Path) -> Context {
    let file_string = file_to_string(file_path);

    let parsed: Context = serde_yaml::from_str(file_string.as_str()).unwrap_or_else(|_| {
      println!(
        "Jira session configuration is invalid. Please make sure this is a valid JIRA cli project"
      );
      std::process::exit(2);
    });

    parsed
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
  pub email: String,
  pub api_key: String,
  pub domain: String,
}

impl Session {
  pub fn load(file_path: &Path) -> Session {
    let file_string = file_to_string(&file_path);

    let parsed: Session = serde_yaml::from_str(file_string.as_str()).unwrap_or_else(|_| {
      println!(
        "Jira session configuration is invalid. Please make sure this is a valid JIRA cli project"
      );
      std::process::exit(2);
    });

    parsed
  }

  pub fn persist(&self, file_path: &Path) {
    let serialized = serde_yaml::to_string(&self).expect("Unable to save configuration.");
    write_file(file_path, serialized);
  }
}