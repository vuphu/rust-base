use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateTodoRequest {
    pub title: String,
    pub due_date: DateTime<Utc>,
}
