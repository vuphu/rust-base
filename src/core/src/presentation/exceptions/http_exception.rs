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

#[derive(Serialize)]
struct JsonError {
    status: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<HashMap<String, Vec<String>>>,
}

impl HttpException {
    fn translated_message(&self) -> String {
        let raw = match self {
            HttpException::Application(err) => err.to_string(),
            err => err.to_string(),
        };
        t!(&raw).to_string()
    }

    fn validation_fields(&self) -> Option<HashMap<String, Vec<String>>> {
        let HttpException::UnprocessableEntity(fields) = self else {
            return None;
        };

        let fields = fields
            .iter()
            .map(|(field, errors)| {
                let translated = errors
                    .iter()
                    .map(|e| {
                        let key = format!("validation.{}", e);
                        let result = t!(&key);
                        if result == key {
                            t!("validation.invalid").to_string()
                        } else {
                            result.to_string()
                        }
                    })
                    .collect();
                (field.clone(), translated)
            })
            .collect();

        Some(fields)
    }

    fn to_json_error(&self) -> JsonError {
        JsonError {
            status: self.status_code().as_u16(),
            message: self.translated_message(),
            fields: self.validation_fields(),
        }
    }
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
        if let HttpException::Application(Exception::InternalServerError(err)) = self {
            error!("Internal Server Error: {}", err);
        }
        HttpResponse::build(self.status_code()).json(self.to_json_error())
    }
}
