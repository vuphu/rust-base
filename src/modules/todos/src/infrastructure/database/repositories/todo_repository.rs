use crate::domain::entities::todo_entity::TodoEntity;
use crate::domain::repositories::todo_repository::TodoRepository;
use crate::infrastructure::database::models::todo_model;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use shared::{BaseRepository, Error};
use std::sync::Arc;

pub struct TodoRepositoryImpl {
    connection: Arc<DatabaseConnection>,
}

impl TodoRepositoryImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl BaseRepository<TodoEntity> for TodoRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<TodoEntity>, Error> {
        let todos = todo_model::Entity::find().all(&*self.connection).await?;
        Ok(todos.into_iter().map(|m| m.into()).collect())
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn create_todo(
        &self,
        title: String,
        due_date: NaiveDateTime,
    ) -> Result<TodoEntity, Error> {
        let todo_draft = todo_model::ActiveModel {
            title: Set(title),
            due_date: Set(due_date),
            ..Default::default()
        };

        let todo = todo_model::Entity::insert(todo_draft)
            .exec_with_returning(&*self.connection)
            .await?;

        Ok(todo.into())
    }
}
