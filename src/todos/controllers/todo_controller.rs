use actix_web::{get, web, Responder};

use crate::todos::services::todo_service::{TodoService, TodoServiceImpl};

pub fn configure(config: &mut web::ServiceConfig) {
    let todo_service: Box<dyn TodoService> = Box::new(TodoServiceImpl::new());
    config.service(
        web::scope("/todos")
            .app_data(web::Data::new(todo_service))
            .service(get_todos),
    );
}

#[get("")]
async fn get_todos(todo_service: web::Data<Box<dyn TodoService>>) -> impl Responder {
    web::Json(todo_service.get_todos().await)
}
