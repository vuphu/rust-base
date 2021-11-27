use async_trait::async_trait;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::core::common::error::AppError;
use crate::core::repositories::base_repository::BaseRepository;
use crate::todos::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoRepository: BaseRepository + Sync + Send {
    async fn find(&self) -> Result<Vec<TodoEntity>, AppError>;
    async fn create(&self, entity: TodoEntity) -> Result<TodoEntity, AppError>;
}

impl<T> BaseRepository for T where T: TodoRepository {}

/// TodoRepositoryImpl
pub struct TodoRepositoryImpl {
    todos: Arc<Mutex<Vec<TodoEntity>>>,
}

impl TodoRepositoryImpl {
    pub fn new() -> Self {
        return TodoRepositoryImpl {
            todos: Arc::new(Mutex::from(vec![])),
        };
    }
}

/// TodoMongoRepository
pub struct TodoMongoRepository {}

impl TodoMongoRepository {
    pub fn new() -> Self {
        return TodoMongoRepository {};
    }
}

#[async_trait]
impl TodoRepository for TodoMongoRepository {
    async fn find(&self) -> Result<Vec<TodoEntity>, AppError> {
        return Ok(Vec::new());
    }

    async fn create(&self, entity: TodoEntity) -> Result<TodoEntity, AppError> {
        return Ok(entity);
    }
}

/// TodoSQLRepository
pub struct TodoSQLRepository {}

impl TodoSQLRepository {
    pub fn new() -> Self {
        return TodoSQLRepository {};
    }
}

#[async_trait]
impl TodoRepository for TodoSQLRepository {
    async fn find(&self) -> Result<Vec<TodoEntity>, AppError> {
        return Ok(Vec::new());
    }

    async fn create(&self, entity: TodoEntity) -> Result<TodoEntity, AppError> {
        return Ok(entity);
    }
}
