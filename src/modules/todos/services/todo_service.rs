use crate::common::base::error::AppError;
use crate::common::services::base_service::BaseService;
use crate::modules::todos::dto::requests::create_todo_request::CreateTodoRequestDto;
use crate::modules::todos::models::todo;
use crate::modules::todos::repositories::todo_repository::TodoRepository;

pub struct TodoService {
    pub todo_repository: TodoRepository,
}

impl BaseService for TodoService {}

impl TodoService {
    pub fn new() -> Self {
        return TodoService {
            todo_repository: TodoRepository::new(),
        };
    }
}

impl TodoService {
    pub async fn get_todos(&self) -> Result<Vec<todo::Model>, AppError> {
        self.todo_repository.find().await
    }

    pub async fn create_todo(&self, dto: CreateTodoRequestDto) -> Result<todo::Model, AppError> {
        let new_todo = self
            .todo_repository
            .create_one(dto.title, dto.due_date)
            .await?;
        Ok(new_todo)
    }
}
