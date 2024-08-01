use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;

mod common;
mod config;
mod modules;

#[get("/")]
async fn index() -> impl Responder {
    return "Hello, world!";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    config::mongodb::setup_mongo().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(|cfg| modules::todos::configure(cfg))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
