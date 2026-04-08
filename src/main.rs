use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use shared::{Env, request_context_middleware};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "todos", description = "Todo management.")
    ),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    shared::initialize().await;

    let mut api_doc = ApiDoc::openapi();
    api_doc.merge(shared::ApiDoc::openapi());
    api_doc.merge(todos::ApiDoc::openapi());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_doc.clone()))
            .wrap(from_fn(request_context_middleware))
            .configure(|config| shared::configure(config))
            .configure(|config| todos::configure(config, shared::get_db_connection()))
    })
    .bind(format!("0.0.0.0:{}", Env::instance().app_port))?
    .run()
    .await
}
