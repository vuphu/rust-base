use crate::domain::entities::todo_entity::TodoEntity;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TodoResponse {
    id: Uuid,
    title: String,
    due_date: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<TodoEntity> for TodoResponse {
    fn from(entity: TodoEntity) -> Self {
        Self {
            id: entity.id,
            title: entity.title.clone(),
            due_date: entity.due_date,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
