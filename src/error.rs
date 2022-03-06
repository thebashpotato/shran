use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShranError<'e> {
    #[error("Error: {msg:?} file does not exist\nFile: {file:?}\nLine: {line:?}")]
    BuildFileError {
        msg: String,
        file: &'e str,
        line: u32,
    },
    #[error("Error: {msg:?} Build option conflict\nFile: {file:?}\nLine: {line:?}")]
    BuildStrategyError {
        msg: String,
        file: &'e str,
        line: u32,
    },
}

pub type ShranErrorType<'e, T> = Result<T, ShranError<'e>>;
