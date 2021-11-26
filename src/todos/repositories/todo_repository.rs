use async_trait::async_trait;

use crate::core::repositories::base_repository::BaseRepository;
use crate::todos::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoRepository: BaseRepository + Sync + Send {
    async fn get_todos(&self) -> Vec<TodoEntity>;
}

impl<T> BaseRepository for T where T: TodoRepository {}

pub struct TodoRepositoryImpl {}

impl TodoRepositoryImpl {
    pub fn new() -> Self {
        return TodoRepositoryImpl {};
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_todos(&self) -> Vec<TodoEntity> {
        let mut result = Vec::new();
        result.push(TodoEntity::new(String::from("First todo")));
        result.push(TodoEntity::new(String::from("Second todo")));
        result.push(TodoEntity::new(String::from("Third todo")));
        return result;
    }
}
