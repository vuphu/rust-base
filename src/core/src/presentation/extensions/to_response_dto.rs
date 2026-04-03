use crate::presentation::exceptions::http_exception::HttpException;

pub trait ToResponseDto<T, E> {
    fn response<F, R>(self, mapper: F) -> Result<R, HttpException>
    where
        F: FnOnce(T) -> R,
        HttpException: From<E>;

    fn vec_response<F, R, I>(self, mapper: F) -> Result<Vec<R>, HttpException>
    where
        T: IntoIterator<Item = I>,
        F: FnMut(I) -> R,
        HttpException: From<E>;
}

impl<T, E> ToResponseDto<T, E> for Result<T, E> {
    fn response<F, R>(self, mapper: F) -> Result<R, HttpException>
    where
        F: FnOnce(T) -> R,
        HttpException: From<E>,
    {
        self.map(mapper).map_err(HttpException::from)
    }

    fn vec_response<F, R, I>(self, mapper: F) -> Result<Vec<R>, HttpException>
    where
        T: IntoIterator<Item = I>,
        F: FnMut(I) -> R,
        HttpException: From<E>,
    {
        self.map(|val| val.into_iter().map(mapper).collect())
            .map_err(HttpException::from)
    }
}
