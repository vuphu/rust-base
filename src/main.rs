use crate::settings::docs_setting::ApiDoc;
use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod common;
mod modules;
mod settings;

#[get("/")]
async fn index() -> impl Responder {
    return "Hello, world!";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    settings::database_setting::setting().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(|cfg| modules::todos::configure(cfg))
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
