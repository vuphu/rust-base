use thiserror::Error;

#[derive(Error, Debug)]
pub enum Exception {
    #[error("{0} not found")]
    EntityNotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError(String),
}

impl From<crate::domain::errors::Error> for Exception {
    fn from(err: crate::domain::errors::Error) -> Self {
        use crate::domain::errors::Error::*;
        match err {
            InternalError(err) => Exception::InternalServerError(err),
            EntityNotFound(err) => Exception::EntityNotFound(err),
        }
    }
}
