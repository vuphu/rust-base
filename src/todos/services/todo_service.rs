use async_trait::async_trait;

use crate::todos::entities::todo_entity::TodoEntity;
use crate::todos::repositories::todo_repository::{TodoRepository, TodoRepositoryImpl};

#[async_trait]
pub trait TodoService: Sync + Send {
    async fn get_todos(&self) -> Vec<TodoEntity>;
}

pub struct TodoServiceImpl {
    pub todo_repository: Box<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new() -> Self {
        return TodoServiceImpl {
            todo_repository: Box::new(TodoRepositoryImpl::new()),
        };
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_todos(&self) -> Vec<TodoEntity> {
        return self.todo_repository.get_todos().await;
    }
}
