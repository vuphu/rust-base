mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::web;
use domain::repositories::todo_repository::TodoRepository;
use infrastructure::database::repositories::todo_repository::TodoRepositoryImpl;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub use presentation::document::ApiDoc;

pub fn configure(config: &mut web::ServiceConfig, db_connection: Arc<DatabaseConnection>) {
    let todo_repository: Arc<dyn TodoRepository> = Arc::new(TodoRepositoryImpl::new(db_connection));

    config.app_data(web::Data::new(todo_repository));
    presentation::controllers::todo_controller::configure(config);
}
