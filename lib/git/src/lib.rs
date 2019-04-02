use std::error::Error;
use std::result::Result as RustResult;
use std::path::Path;
use git2::Repository;

type Result<T> = RustResult<T, Box<Error>>;

pub fn get_repository_context(path_to_repo: &Path) -> Result<Repository> {
    // This crawls upwards until we find a repo
    Ok(Repository::discover(&path_to_repo)?)
}

pub fn create_branch(name: &str, repo: &Repository) {

}