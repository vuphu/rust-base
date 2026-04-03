use crate::application::use_cases::create_todo_use_case::{CreateTodoInput, CreateTodoUseCase};
use crate::application::use_cases::get_todos_use_case::GetTodosUseCase;
use crate::domain::repositories::todo_repository::TodoRepository;
use crate::presentation::dtos::requests::create_todo_request::CreateTodoRequest;
use crate::presentation::dtos::responses::todo_response::TodoResponse;
use actix_web::{HttpResponse, get, post, web};
use shared::{HttpException, ToHttpResponse, ToResponseDto, UseCase, ValidatedJson};
use std::sync::Arc;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(web::scope("/todos").service(get_todos).service(create_todo));
}

#[get("")]
pub async fn get_todos(
    todo_repository: web::Data<Arc<dyn TodoRepository>>,
) -> Result<HttpResponse, HttpException> {
    let use_case = GetTodosUseCase::new(todo_repository.get_ref().clone());
    use_case
        .handle(())
        .await
        .vec_response(TodoResponse::from)
        .to_http_response()
}

#[post("")]
async fn create_todo(
    todo_repository: web::Data<Arc<dyn TodoRepository>>,
    dto: ValidatedJson<CreateTodoRequest>,
) -> Result<HttpResponse, HttpException> {
    let dto = dto.into_inner();
    let use_case = CreateTodoUseCase::new(todo_repository.get_ref().clone());
    use_case
        .handle(CreateTodoInput {
            title: dto.title,
            due_date: dto.due_date,
        })
        .await
        .response(TodoResponse::from)
        .to_http_response()
}
