mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::config::env_config;
use crate::infrastructure::config::trace_config;
use crate::infrastructure::database::sql_connection;
use actix_web::web;

pub use application::exceptions::Exception;
pub use application::use_cases::base_use_case::UseCase;
pub use domain::entities::base_entity::BaseEntity;
pub use domain::errors::Error;
pub use domain::repositories::base_repository::BaseRepository;
pub use infrastructure::config::env_config::Env;
pub use infrastructure::database::sql_connection::get_db_connection;
pub use presentation::exceptions::http_exception::HttpException;
pub use presentation::extensions::to_http_response::ToHttpResponse;
pub use presentation::extensions::to_response_dto::ToResponseDto;
pub use presentation::extractors::validated_json::ValidatedJson;

pub async fn initialize() {
    env_config::initialize();
    trace_config::initialize();
    sql_connection::initialize().await;
}

pub fn configure(_: &mut web::ServiceConfig) {
    // No-op
}
