use crate::domain::entities::todo_entity::TodoEntity;
use crate::domain::repositories::todo_repository::TodoRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{Exception, UseCase, ensure_request, utils::is_future};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct CreateTodoInput {
    pub title: String,
    pub due_date: DateTime<Utc>,
}

pub struct CreateTodoUseCase {
    todo_repository: Arc<dyn TodoRepository>,
}

impl CreateTodoUseCase {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl UseCase<CreateTodoInput, TodoEntity> for CreateTodoUseCase {
    async fn handle(&self, input: CreateTodoInput) -> Result<TodoEntity, Exception> {
        ensure_request!(is_future(input.due_date), "error.modules.todos.due_date_in_past");
        let todo = self.todo_repository.create_todo(input.title, input.due_date).await?;
        Ok(todo)
    }
}
