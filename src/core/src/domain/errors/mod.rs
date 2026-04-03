use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InternalError(String),
    #[error("{0} not found")]
    EntityNotFound(String),
}
