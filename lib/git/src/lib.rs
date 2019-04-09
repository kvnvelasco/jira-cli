// For the moment these are simple calls to the command line;
// Eventually we want to replace this with the git2 repository and
// handle all the possible failure conditions. At the moment,
// the existing git cli covers 90% of the cases required to fulfil the
// use case of the workflow-cli tool

// use git2::Repository;
use std::process::{Command, Stdio};

pub fn create_branch_on_master(branch_name: &str) {
  let branch_exists = Command::new("git")
    .arg("rev-parse")
    .arg("--verify")
    .arg(branch_name)
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute command")
    .wait_with_output()
    .expect("Failed")
    .status
    .success();
  //
  if (branch_exists) {
    checkout_branch(&branch_name);
  };
}

pub fn checkout_branch(branch_name: &str) -> bool {
  Command::new("git")
    .arg("checkout")
    .arg(branch_name)
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute command")
    .wait_with_output()
    .expect("Failed")
    .status
    .success()
}

pub fn create_branch(branch_name: &str) -> bool {
  Command::new("git")
    .arg("checkout")
    .arg("-b")
    .arg(branch_name)
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute command")
    .wait_with_output()
    .expect("Failed")
    .status
    .success()
}
// fetch latest master and pull from upstream
