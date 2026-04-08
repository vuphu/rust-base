rust_i18n::i18n!("../locales");

mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::web;
use infrastructure::config::env_config;
use infrastructure::config::i18n_config;
use infrastructure::config::trace_config;
use infrastructure::database::sql_connection;

pub use application::exceptions::Exception;
pub use application::use_cases::base_use_case::UseCase;
pub use domain::entities::base_entity::BaseEntity;
pub use domain::errors::Error;
pub use domain::repositories::base_repository::BaseRepository;
pub use infrastructure::config::env_config::Env;
pub use infrastructure::database::sql_connection::get_db_connection;
pub use presentation::document::ApiDoc;
pub use presentation::exceptions::http_exception::HttpException;
pub use presentation::extensions::to_http_response::ToHttpResponse;
pub use presentation::extensions::to_response_dto::ToResponseDto;
pub use presentation::extractors::request_context::RequestContext;
pub use presentation::extractors::validated_json::ValidatedJson;
pub use presentation::middlewares::request_context_middleware::request_context_middleware;

pub async fn initialize() {
    env_config::initialize();
    trace_config::initialize();
    i18n_config::initialize();
    sql_connection::initialize().await;
}

pub fn configure(config: &mut web::ServiceConfig) {
    presentation::controllers::app_controller::configure(config);
    presentation::document::configure(config);
}
