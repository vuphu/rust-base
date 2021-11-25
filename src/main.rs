use actix_web::{get, App, HttpServer, Responder};

mod core;
mod todos;

#[get("/")]
async fn index() -> impl Responder {
    return "Hello, world!";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(|cfg| todos::configure(cfg))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
