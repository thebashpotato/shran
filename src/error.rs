use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShranError<'error> {
    #[error("Error: {msg:?} file does not exist\nFile: {file:?} [{line:?}:{column:?}]")]
    BuildFileError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?} does not match\nFile: {file:?} [{line:?}:{column:?}]")]
    UnrecognizedBuildOptionNameError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    GithubTokenNotFoundError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    GithubTokenReadError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    BlockchainVersionAlreadyExistsError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
    #[error("Error: {msg:?}\nFile: {file:?} [{line:?}:{column:?}]")]
    ManifestEntryError {
        msg: String,
        file: &'error str,
        line: u32,
        column: u32,
    },
}
