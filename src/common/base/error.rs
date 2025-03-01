use err_derive::Error;
use sea_orm::DbErr;
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

impl std::convert::From<sea_orm::DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        AppError::internal_error(format!("{}", value))
    }
}

impl std::convert::From<dotenv::Error> for AppError {
    fn from(err: dotenv::Error) -> Self {
        AppError::internal_error(format!("{:?}", err))
    }
}
