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

impl Exception {
    pub fn entity_not_found<S: Into<String>>(msg: S) -> Self {
        Self::EntityNotFound(msg.into())
    }

    pub fn bad_request<S: Into<String>>(msg: S) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn internal_error<S: Into<String>>(msg: S) -> Self {
        Self::InternalServerError(msg.into())
    }
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
