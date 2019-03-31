use crate::utils::fs::{create_dir, create_file, get_absolute_path};
use crate::utils::io::read_line;

use crate::utils::config::{Paths, Session};
use std::fs;
use std::path::PathBuf;

pub fn start(path_to_init: Option<&str>) {
  let path = PathBuf::from(path_to_init.unwrap_or(""));
  let working_directory = get_absolute_path(path);
  let paths = Paths::new(&working_directory, false);

  fs::create_dir_all(&paths.config_dir).expect("Unable to create configuration directory");
  // // // create a configuration files
  if let Some(path) = &paths
    .config_dir
    .canonicalize()
    .expect("Unable to resolve path to directory")
    .to_str()
  {
    println!("Created Directory at, {}", &path)
  }

  create_dir(&paths.boards_dir);
  create_dir(&paths.issues_dir);

  create_file(&paths.config_file);
  create_file(&paths.context_file);
  create_file(&paths.session_file);

  let domain = read_line("Provide your atlassian domain (e.g: company.atlassian.net)");
  let email = read_line("Enter your email");

  println!(
    "Enter your atlassian API key. \
     \nFor instructions on how to generate an api key, visit \
     \nhttps://confluence.atlassian.com/cloud/api-tokens-938839638.html"
  );

  let api_key = read_line("Your API key");

  let session = Session {
    email: email,
    domain: domain,
    api_key: api_key,
  };

  session.persist(&paths.session_file);
}
