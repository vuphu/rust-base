use err_derive::Error;
use serde::Serialize;

#[derive(Serialize, Debug, Error)]
pub enum AppError {
    #[error(display = "{}", message)]
    InternalError { message: String },
    #[error(display = "{}", message)]
    BadUserInput { message: String },
}
