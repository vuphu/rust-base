use chrono::{serde::ts_milliseconds, serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::entities::base_entity::BaseEntity;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoEntity {
    pub id: String,
    pub title: String,
    #[serde(with = "ts_milliseconds_option")]
    #[serde(default)]
    pub deadline: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

impl BaseEntity for TodoEntity {}

impl TodoEntity {
    pub fn new(title: String, deadline: Option<DateTime<Utc>>) -> Self {
        return TodoEntity {
            id: Uuid::new_v4().to_string(),
            title,
            deadline,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
    }
}
