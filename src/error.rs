use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShranError<'e> {
    #[error("Error: {msg:?} file does not exist\nFile: {file:?} [{line:?}:{column:?}]")]
    BuildFileError {
        msg: String,
        file: &'e str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?} does not match\nFile: {file:?} [{line:?}:{column:?}]")]
    UnrecognizedBuildOptionNameError {
        msg: String,
        file: &'e str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    GithubTokenNotFoundError {
        msg: String,
        file: &'e str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    GithubTokenReadError {
        msg: String,
        file: &'e str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    BlockchainVersionAlreadyExistsError {
        msg: String,
        file: &'e str,
        line: u32,
        column: u32,
    },
}
