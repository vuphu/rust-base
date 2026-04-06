use crate::presentation::exceptions::http_exception::HttpException;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use std::future::{Ready, ready};

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub locale: String,
}

impl FromRequest for RequestContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let res = req
            .extensions()
            .get::<RequestContext>()
            .cloned()
            .ok_or_else(|| {
                HttpException::InternalServerError("Required request context not found".to_string())
            });
        ready(res.map_err(|e| e.into()))
    }
}
