use crate::Error;
use sea_orm::DbErr;

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Error::InternalError(err.to_string())
    }
}
