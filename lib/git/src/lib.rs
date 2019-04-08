use git2::Repository;
use std::error::Error;
use std::path::Path;
use std::result::Result as RustResult;

type Result<T> = RustResult<T, Box<Error>>;

pub fn get_repository_context(path_to_repo: &Path) -> Result<Repository> {
    // This crawls upwards until we find a repo
    Ok(Repository::discover(&path_to_repo)?)
}

pub fn create_branch(_name: &str, _repo: &Repository) {}
