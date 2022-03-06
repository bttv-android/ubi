use std::sync::PoisonError;
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
    #[error("class interface with missing interface path found in this line: {0}")]
    MissingInterfacePath(String),
    #[error(".class declaration not found")]
    MissingClass(),
    #[error("multiple .class declarations found")]
    TooManyClasses(),
    #[error("multiple .super declarations found")]
    TooManySupers(),
    #[error("PoisonedLockError: {0:#?}")]
    PoisonedLockError(String),
}

impl<T> From<PoisonError<T>> for ParserError {
    fn from(err: PoisonError<T>) -> Self {
        Self::PoisonedLockError(err.to_string())
    }
}

pub type ParserResult<T> = Result<T, ParserError>;
