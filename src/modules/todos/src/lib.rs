mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::domain::repositories::todo_repository::TodoRepository;
use crate::infrastructure::database::repositories::todo_repository::TodoRepositoryImpl;
use actix_web::web;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn configure(config: &mut web::ServiceConfig, db_connection: Arc<DatabaseConnection>) {
    let todo_repository: Arc<dyn TodoRepository> = Arc::new(TodoRepositoryImpl::new(db_connection));

    config.app_data(web::Data::new(todo_repository));
    presentation::controllers::todo_controller::configure(config);
}
