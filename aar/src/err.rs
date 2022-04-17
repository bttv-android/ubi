use std::{fmt::Result, process::Output};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrepareAARError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("ZipError: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("dx failed: {0:?}")]
    DXErr(Output),
}

pub type PrepareAARResult<T> = std::result::Result<T, PrepareAARError>;
