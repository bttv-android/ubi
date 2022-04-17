use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("PrepareAARError")]
    PrepareAARError(#[from] aar::PrepareAARError),
}
