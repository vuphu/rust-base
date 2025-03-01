use crate::common::base::error::AppError;
use crate::common::repositories::base_repository::BaseRepository;
use crate::modules::todos::models::todo;
use crate::settings::database_setting::DB_POOL;
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{DbErr, EntityTrait};

pub struct TodoRepository {}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {}
    }
}

impl BaseRepository for TodoRepository {}

impl TodoRepository {
    pub async fn find(&self) -> Result<Vec<todo::Model>, AppError> {
        let connection = DB_POOL.get().unwrap();
        let todos = todo::Entity::find().all(connection).await?;
        Ok(todos)
    }

    pub async fn create_one(
        &self,
        title: String,
        due_date: NaiveDateTime,
    ) -> Result<todo::Model, DbErr> {
        let connection = DB_POOL.get().unwrap();
        let partial_entity = todo::ActiveModel {
            title: Set(title),
            due_date: Set(due_date),
            ..Default::default()
        };

        let inserted_result = todo::Entity::insert(partial_entity)
            .exec(connection)
            .await?;
        let todo = todo::Entity::find_by_id(inserted_result.last_insert_id)
            .one(connection)
            .await?;

        Ok(todo.unwrap())
    }
}
