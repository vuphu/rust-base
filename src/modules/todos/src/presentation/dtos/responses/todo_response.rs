use crate::domain::entities::todo_entity::TodoEntity;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct TodoResponse {
    id: Uuid,
    title: String,
    due_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
