use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;

use crate::common::base::error::AppError;
use crate::common::entities::base_entity::BaseEntity;

#[async_trait]
pub trait BaseMongoRepository<T: 'static + BaseEntity>: Sync + Send {
    fn collection(&self) -> &Collection<T>;

    async fn find(&self) -> Result<Vec<T>, AppError> {
        let mut cursor = self.collection().find(doc! {}, None).await?;

        let mut result: Vec<T> = vec![];
        while let Some(value) = cursor.try_next().await? {
            result.push(value)
        }

        Ok(result)
    }

    async fn create(&self, entity: T) -> Result<T, AppError> {
        self.collection().insert_one(entity.clone(), None).await?;
        Ok(entity)
    }
}
