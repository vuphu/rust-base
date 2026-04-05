use crate::domain::entities::todo_entity::TodoEntity;
use async_trait::async_trait;
use shared::{BaseRepository, Error};

#[async_trait]
pub trait TodoRepository: BaseRepository<TodoEntity> {
    async fn create_todo(
        &self,
        title: String,
        due_date: chrono::NaiveDateTime,
    ) -> Result<TodoEntity, Error>;
}
