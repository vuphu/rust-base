use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateTodoRequest {
    #[validate(length(max = 255))]
    pub title: String,
    pub due_date: DateTime<Utc>,
}
