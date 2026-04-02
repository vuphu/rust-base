use crate::domain::entities::todo_entity::TodoEntity;
use crate::domain::repositories::todo_repository::TodoRepository;
use async_trait::async_trait;
use shared::{Exception, UseCase};
use std::sync::Arc;

pub struct GetTodosUseCase {
    todo_repository: Arc<dyn TodoRepository>,
}

impl GetTodosUseCase {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl UseCase<(), Vec<TodoEntity>> for GetTodosUseCase {
    async fn handle(&self, _: ()) -> Result<Vec<TodoEntity>, Exception> {
        let todos = self.todo_repository.find_all().await?;
        Ok(todos)
    }
}
