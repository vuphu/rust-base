use crate::modules::todos::dto::requests::create_todo_request::CreateTodoRequestDto;
use crate::modules::todos::dto::responses::todo_response::TodoResponseDto;
use crate::modules::todos::services::todo_service::TodoService;
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn configure(config: &mut web::ServiceConfig) {
    let todo_service: TodoService = TodoService::new();
    config.service(
        web::scope("/todos")
            .app_data(web::Data::new(todo_service))
            .service(get_todos)
            .service(create_todo),
    );
}

#[utoipa::path(
    get,
    path = "/todos",
    tag="todos",
    responses((status = 200, body = [TodoResponseDto]))
)]
#[get("")]
pub async fn get_todos(todo_service: web::Data<TodoService>) -> impl Responder {
    match todo_service.get_todos().await {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    tag = "todos",
    responses((status = 200, body = TodoResponseDto)),
)]
#[post("")]
async fn create_todo(
    todo_service: web::Data<TodoService>,
    input: web::Json<CreateTodoRequestDto>,
) -> impl Responder {
    match todo_service.create_todo(input.into_inner()).await {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
