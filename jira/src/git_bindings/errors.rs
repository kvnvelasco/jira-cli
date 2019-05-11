#[derive(Debug, Clone)]
pub enum Errors {
    InvalidRepo,
    BranchNotSet,
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Errors::InvalidRepo => write!(f, "Git Error: Directory is not a valid git repository"),
            Errors::BranchNotSet => write!(f, "Git error: Active branch is not set"),
        };
        Ok(())
    }
}

impl std::error::Error for Errors {}
