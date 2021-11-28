use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;

mod config;
mod core;
mod todos;

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
            .configure(|cfg| todos::configure(cfg))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
