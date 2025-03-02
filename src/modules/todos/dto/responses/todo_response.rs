use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct TodoResponseDto {
    id: String,
    title: String,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
