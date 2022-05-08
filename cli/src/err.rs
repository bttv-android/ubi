use common::thiserror;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ApplicationError {
    #[error(transparent)]
    PrepareAARError(#[from] aar::PrepareAARError),
}
