use crate::git_bindings::errors::Errors;
use git2::{self, BranchType};
use std::error::Error as StdErr;

mod errors;
mod tests;

type StandardError = Box<StdErr>;

pub struct Repository {
    repo: git2::Repository,
    remote: Option<String>,
    ref_spec: Option<String>,
    // the default branch root when creating new branches
    branch_root: String,
}

impl Repository {
    pub fn new(path: &std::path::Path) -> Result<Repository, errors::Errors> {
        let repo = git2::Repository::open(path).or(Err(errors::Errors::InvalidRepo))?;
        Ok(Repository {
            repo,
            remote: None,
            ref_spec: None,
            branch_root: "master".to_string(),
        })
    }

    // It's entirely possible that the current head does not point to a branch
    //    pub fn current_branch_name(&self) -> Option<String> {
    //        let repo = &self.repo;
    //        let head = repo.head().unwrap();
    //
    //        Some(head.name()?.to_string())
    //    }

    fn current_branch(&self) -> Result<git2::Branch, errors::Errors> {
        if let Some(branch_name) = &self.ref_spec {
            self.repo
                .find_branch(branch_name, git2::BranchType::Local)
                .or(Err(errors::Errors::BranchNotSet))
        } else {
            Err(errors::Errors::BranchNotSet)
        }
    }

    pub fn set_remote(&mut self, remote: &str) -> &mut Self {
        self.remote = Some(remote.to_owned());
        self
    }

    pub fn branch_exists(&self, branch: &str) -> bool {
        self.repo.find_branch(&branch, BranchType::Local).is_ok()
    }

    pub fn set_branch(&mut self, branch: &str) -> Result<&mut Self, errors::Errors> {
        if self.branch_exists(&branch) {
            self.ref_spec = Some(branch.to_owned());
            self.checkout()?;
        };
        Ok(self)
    }

    pub fn create_branch(&mut self, branch: &str) -> Result<&mut Self, StandardError> {
        {
            let object = &self
                .repo
                .revparse_single(&self.branch_root)?
                .peel_to_commit()?;
            self.repo.branch(branch, object, false)?;
            self.ref_spec = Some(branch.to_owned());
        }
        Ok(self)
    }

    // Checkout the current ref_spec pointed pointed by the repo HEAD
    fn checkout(&mut self) -> Result<&mut Self, errors::Errors> {
        {
            let branch = self.current_branch()?.into_reference();
            self.repo
                .set_head(branch.name().expect("Invalid Branch name"))
                .unwrap();
            self.repo.checkout_head(None).unwrap();
        }
        {
            // We want to make sure that the index is always set to the head.
            self.reset_index();
        }
        Ok(self)
    }

    //    fn stash_index(&mut self) -> Result<&mut Self, errors::Errors> {
    //        let repo = self.repo.stash_save()
    //    }

    fn reset_index(&mut self) -> Result<&mut Self, errors::Errors> {
        {
            let head = self
                .repo
                .head()
                .or(Err(Errors::InvalidRepo))?
                .peel(git2::ObjectType::Any)
                .or(Err(Errors::InvalidRepo))?;
            self.repo
                .reset(&head, git2::ResetType::Hard, None)
                .or(Err(Errors::InvalidRepo))?;
        }

        Ok(self)
    }
}
