use std::process::Output;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum PrepareAARError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("dx failed: {0:?}")]
    DXErr(Output),
}

pub type PrepareAARResult<T> = std::result::Result<T, PrepareAARError>;
