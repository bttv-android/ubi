use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("class with missing class path found in this line: {0}")]
    MissingClassPath(String),
    #[error("invalid class path found: {0}")]
    InvalidClassPath(String),
}

pub type ParserResult<T> = Result<T, ParserError>;
