use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateTodoRequestDto {
    pub title: String,
    pub due_date: NaiveDateTime,
}
