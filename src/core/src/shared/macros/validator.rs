#[macro_export]
macro_rules! ensure_request {
    ($cond:expr, $key:expr) => {
        if !$cond {
            return Err(shared::Exception::BadRequest($key.into()));
        }
    };
}
