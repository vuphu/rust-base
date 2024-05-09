use crate::config::mongodb::get_database;
use async_trait::async_trait;
use mongodb::Collection;

use crate::common::repositories::base_mongo_repository::BaseMongoRepository;
use crate::modules::todos::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoRepository: BaseMongoRepository<TodoEntity> + Sync + Send {}

/// TodoMongoRepository
pub struct TodoMongoRepository {
    collection: Collection<TodoEntity>,
}

impl TodoMongoRepository {
    pub fn new() -> Self {
        let collection = get_database().unwrap().collection::<TodoEntity>("todos");
        return TodoMongoRepository { collection };
    }
}

impl BaseMongoRepository<TodoEntity> for TodoMongoRepository {
    fn collection(&self) -> &Collection<TodoEntity> {
        return &self.collection;
    }
}

#[async_trait]
impl TodoRepository for TodoMongoRepository {}

// SQL implementation

// #[async_trait]
// pub trait TodoRepository: BaseSQLRepository<TodoEntity> + Sync + Send {}

// /// TodoSQLRepository
// pub struct TodoSQLRepository {}
//
// impl TodoSQLRepository {
//     pub fn new() -> Self {
//         return TodoSQLRepository {};
//     }
// }
//
// #[async_trait]
// impl BaseSQLRepository<TodoEntity> for TodoSQLRepository {}
