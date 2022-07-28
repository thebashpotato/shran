use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShranError<'e> {
    #[error("Error: {msg:?} file does not exist\nFile: {file:?}\nLine: {line:?}")]
    BuildFileError {
        msg: String,
        file: &'e str,
        line: u32,
    },
    #[error("Error: {msg:?} does not match\nFile: {file:?}\nLine: {line:?}")]
    UnrecognizedBuildOptionNameError {
        msg: String,
        file: &'e str,
        line: u32,
    },
}
