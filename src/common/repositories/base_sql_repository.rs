use async_trait::async_trait;

use crate::common::base::error::AppError;
use crate::common::entities::base_entity::BaseEntity;

#[async_trait]
pub trait BaseSQLRepository<T: 'static + BaseEntity>: Sync + Send {
    async fn find(&self) -> Result<Vec<T>, AppError> {
        return Err(AppError::oops());
    }

    async fn create(&self, _entity: T) -> Result<T, AppError> {
        return Err(AppError::oops());
    }
}
