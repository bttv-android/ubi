use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrepareAARError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("ZipError: {0}")]
    ZipError(#[from] zip::result::ZipError),
}
