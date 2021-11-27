use chrono::{DateTime, serde::ts_milliseconds_option, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    #[serde(with = "ts_milliseconds_option")]
    #[serde(default)]
    pub deadline: Option<DateTime<Utc>>,
}
