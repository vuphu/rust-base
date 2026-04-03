use actix_web::{App, HttpServer, Responder, get};
use shared::Env;

#[get("/")]
async fn index() -> impl Responder {
    "Do not dwell in the past, do not dream of the future, concentrate the mind on the present moment. - Buddha"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    shared::initialize().await;

    HttpServer::new(move || {
        App::new()
            .service(index)
            .configure(|config| shared::configure(config))
            .configure(|config| todos::configure(config, shared::get_db_connection()))
    })
    .bind(format!("0.0.0.0:{}", Env::instance().app_port))?
    .run()
    .await
}
