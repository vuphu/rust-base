use err_derive::Error;
use serde::Serialize;

#[derive(Serialize, Debug, Error)]
pub enum AppError {
    #[error(display = "{}", message)]
    InternalError { message: String },
}

impl AppError {
    pub fn internal_error(message: String) -> AppError {
        return AppError::InternalError { message };
    }

    pub fn oops() -> AppError {
        return AppError::InternalError {
            message: "Oops! Something went wrongs".to_string(),
        };
    }
}

impl std::convert::From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::internal_error(format!("{:?}", err))
    }
}

impl std::convert::From<dotenv::Error> for AppError {
    fn from(err: dotenv::Error) -> Self {
        AppError::internal_error(format!("{:?}", err))
    }
}
