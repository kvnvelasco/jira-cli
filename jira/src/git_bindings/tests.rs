#[cfg(test)]
use super::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

const TMP_DIR: &'static str = "/Volumes/Projects/tmp/workflow-cli-test";
#[allow(dead_code)]
fn setup() -> String {
    let subdir: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let full_dir = format!("{}-{}", TMP_DIR, subdir);
    //     create a temporary directory somewhere;
    std::fs::create_dir_all(std::path::PathBuf::from(&full_dir)).expect("Setup failed");
    std::process::Command::new("git")
        .args(&["init", &full_dir])
        .output()
        .expect("Setup failed");
    // Add a dummy file and commit it
    std::process::Command::new("touch")
        .args(&[format!("{}/some_file", &full_dir)])
        .output()
        .expect("Setup failed");
    std::process::Command::new("git")
        .args(&["-C", &full_dir, "add", "."])
        .output()
        .expect("Setup failed");
    std::process::Command::new("git")
        .args(&["-C", &full_dir, "commit", "-m", "initial commit"])
        .output()
        .expect("Setup failed");
    full_dir
}

#[allow(dead_code)]
fn teardown(dir: &str) {
    std::process::Command::new("rm")
        .args(&["-rf", dir])
        .output()
        .expect("Teardown failed");
}

#[test]
fn bind_to_existing_repo() {
    let directory = setup();
    {
        let path = std::path::PathBuf::from(&directory);
        let repo = Repository::new(&path).unwrap();
        // We should be on the master branch
        assert!(repo.current_branch_name().unwrap().contains("master"));
    }
    teardown(&directory);
}

#[test]
fn create_new_branch_on_master() {
    let directory = setup();
    {
        let path = std::path::PathBuf::from(&directory);
        let mut repo = Repository::new(&path).unwrap();
        repo.create_branch("test-branch");
        repo.checkout();
        assert!(repo.current_branch_name().unwrap().contains("test-branch"));
    }
    teardown(&directory);
}
