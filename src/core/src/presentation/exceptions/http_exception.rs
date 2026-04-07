use crate::application::exceptions::Exception;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use rust_i18n::t;
use serde::Serialize;
use std::collections::HashMap;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum HttpException {
    #[error(transparent)]
    Application(#[from] Exception),
    #[error("Validation Exception")]
    UnprocessableEntity(HashMap<String, Vec<String>>),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl ResponseError for HttpException {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpException::Application(err) => match err {
                Exception::EntityNotFound(_) => StatusCode::NOT_FOUND,
                Exception::BadRequest(_) => StatusCode::BAD_REQUEST,
                Exception::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            HttpException::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            HttpException::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        #[derive(Serialize)]
        struct JsonError {
            status: u16,
            message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<HashMap<String, Vec<String>>>,
        }

        let response = JsonError {
            status: self.status_code().as_u16(),
            message: t!(match self {
                HttpException::Application(err) => err.to_string(),
                err => err.to_string(),
            })
            .to_string(),
            fields: match self {
                HttpException::UnprocessableEntity(fields) => Some(
                    fields
                        .iter()
                        .map(|(field, errors)| {
                            let translated = errors
                                .iter()
                                .map(|e| t!(format!("validation.{}", e)).into())
                                .collect();
                            (field.clone(), translated)
                        })
                        .collect(),
                ),
                _ => None,
            },
        };

        if let HttpException::Application(Exception::InternalServerError(err)) = self {
            error!("Internal Server Error: {}", err);
        }

        HttpResponse::build(self.status_code()).json(response)
    }
}
