use crate::domain::entities::todo_entity::TodoEntity;
use chrono::Timelike;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    title: String,
    due_date: DateTime,
    created_at: DateTime,
    updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for TodoEntity {
    fn from(model: Model) -> TodoEntity {
        TodoEntity {
            id: model.id,
            title: model.title.clone(),
            due_date: model.due_date.and_utc(),
            created_at: model.created_at.with_nanosecond(0).unwrap().and_utc(),
            updated_at: model.updated_at.with_nanosecond(0).unwrap().and_utc(),
        }
    }
}
