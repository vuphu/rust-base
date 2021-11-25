use chrono::{serde::ts_milliseconds, serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TodoEntity {
    pub id: String,
    pub title: String,
    #[serde(with = "ts_milliseconds_option")]
    pub deadline: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

impl TodoEntity {
    pub fn new(title: String) -> Self {
        return TodoEntity {
            id: Uuid::new_v4().to_string(),
            title,
            deadline: Option::from(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
    }
}
