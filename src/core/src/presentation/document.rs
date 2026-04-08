use crate::presentation::controllers::app_controller::*;
use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, web};
use serde_json::json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct ApiDoc;

pub fn configure(config: &mut ServiceConfig) {
    config
        .route(
            "/docs/openapi.json",
            web::get().to(|api_doc: web::Data<utoipa::openapi::OpenApi>| async move {
                HttpResponse::Ok().json(api_doc.get_ref())
            }),
        )
        .configure(scalar_api_reference::actix_web::config(
            "/docs",
            &json!({
                "url": "/docs/openapi.json",
                "theme": "default",
            }),
        ));
}
