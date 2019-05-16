pub type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    BoardContextUnavailable,
    SprintContextUnavailable,
    ContextUnparseable,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BoardContextUnavailable => write!(
                f,
                "You have not selected a board. Please select a board to continue"
            ),
            Error::SprintContextUnavailable => write!(
                f,
                "You have not selected a sprint. Please select a sprint to continue"
            ),
            //            Error::IssuesNotFetched => write!(
            //                f,
            //                "You haven't fetched any issues yet. Please fetch issues to continue"
            //            ),
            Error::ContextUnparseable => write!(
                f,
                "Unable to parse context. Please delete your .jira folder and start again"
            ),
        }
    }
}

impl std::error::Error for Error {}
