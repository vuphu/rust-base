use async_trait::async_trait;

use crate::core::common::error::AppError;
use crate::core::entities::base_entity::BaseEntity;

#[async_trait]
pub trait BaseSQLRepository<T: 'static + BaseEntity>: Sync + Send {
    async fn find(&self) -> Result<Vec<T>, AppError> {
        return Err(AppError::oops());
    }

    async fn create(&self, _entity: T) -> Result<T, AppError> {
        return Err(AppError::oops());
    }
}
