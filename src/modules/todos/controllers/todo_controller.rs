use actix_web::{get, post, web, HttpResponse, Responder};
use crate::modules::todos::dto::create_todo_dto::CreateTodoDto;
use crate::modules::todos::services::todo_service::{TodoService, TodoServiceImpl};


pub fn configure(config: &mut web::ServiceConfig) {
    let todo_service: Box<dyn TodoService> = Box::new(TodoServiceImpl::new());
    config.service(
        web::scope("/todos")
            .app_data(web::Data::new(todo_service))
            .service(get_todos)
            .service(create_todo),
    );
}

#[get("")]
async fn get_todos(todo_service: web::Data<Box<dyn TodoService>>) -> impl Responder {
    match todo_service.get_todos().await {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[post("")]
async fn create_todo(
    todo_service: web::Data<Box<dyn TodoService>>,
    input: web::Json<CreateTodoDto>,
) -> impl Responder {
    match todo_service.create_todo(input.into_inner()).await {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
