use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("class with missing class path found in this line: {0}")]
    MissingClassPath(String),
    #[error("invalid class path found: {0}")]
    InvalidClassPath(String),
    #[error("class super with missing super path found in this line: {0}")]
    MissingSuperPath(String),
    #[error(".class declaration not found")]
    MissingClass(),
}

pub type ParserResult<T> = Result<T, ParserError>;
