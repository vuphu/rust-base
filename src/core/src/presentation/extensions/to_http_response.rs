use crate::presentation::exceptions::http_exception::HttpException;
use actix_web::HttpResponse;
use serde::Serialize;

pub trait ToHttpResponse<T, E> {
    fn to_http_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize;
}

impl<T, E> ToHttpResponse<T, E> for Result<T, E>
where
    T: Serialize,
    HttpException: From<E>,
{
    fn to_http_response(self) -> Result<HttpResponse, HttpException> {
        self.map(|data| HttpResponse::Ok().json(data))
            .map_err(|err| HttpException::from(err))
    }
}
