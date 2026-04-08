use crate::presentation::exceptions::http_exception::HttpException;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, web};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let bytes_future = web::Bytes::from_request(request, payload);

        Box::pin(async move {
            let bytes = bytes_future.await?;
            let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

            let data: T = serde_path_to_error::deserialize(deserializer).map_err(|error| {
                let mut fields = HashMap::new();
                fields.insert(error.path().to_string(), vec!["invalid_type".to_string()]);
                HttpException::UnprocessableEntity(fields)
            })?;

            data.validate().map_err(|errors| {
                let fields = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let messages = errors
                            .iter()
                            .map(|err| {
                                err.message
                                    .as_ref()
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| err.code.to_string())
                            })
                            .collect::<Vec<String>>();
                        (field.to_string(), messages)
                    })
                    .collect::<HashMap<String, Vec<String>>>();

                HttpException::UnprocessableEntity(fields)
            })?;

            Ok(ValidatedJson(data))
        })
    }
}
