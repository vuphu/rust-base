use crate::presentation::exceptions::http_exception::HttpException;
use actix_web::HttpResponse;
use serde::Serialize;

pub trait ToHttpResponse<T, E> {
    fn ok_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize;

    fn created_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize;

    fn no_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize;
}

impl<T, E> ToHttpResponse<T, E> for Result<T, E>
where
    T: Serialize,
    HttpException: From<E>,
{
    fn ok_response(self) -> Result<HttpResponse, HttpException> {
        self.map(|data| HttpResponse::Ok().json(data))
            .map_err(|err| HttpException::from(err))
    }

    fn created_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize,
    {
        self.map(|data| HttpResponse::Created().json(data))
            .map_err(|err| HttpException::from(err))
    }

    fn no_response(self) -> Result<HttpResponse, HttpException>
    where
        T: Serialize,
    {
        self.map(|_| HttpResponse::NoContent().finish())
            .map_err(|err| HttpException::from(err))
    }
}
