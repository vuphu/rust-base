use crate::config::mongodb::get_database;
use async_trait::async_trait;
use mongodb::Collection;

use crate::common::repositories::base_repository::{BaseRepository, MongoRepository};
use crate::modules::todos::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoRepository: BaseRepository<TodoEntity> + Sync + Send {}

pub struct TodoRepositoryImpl {
    collection: Collection<TodoEntity>,
}

impl TodoRepositoryImpl {
    pub fn new() -> Self {
        let collection = get_database().unwrap().collection::<TodoEntity>("todos");
        return TodoRepositoryImpl { collection };
    }
}

impl BaseRepository<TodoEntity> for TodoRepositoryImpl {}

impl MongoRepository<TodoEntity> for TodoRepositoryImpl {
    fn collection(&self) -> &Collection<TodoEntity> {
        return &self.collection;
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {}

