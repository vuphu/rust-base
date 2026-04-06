use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer};
use shared::{Env, request_context_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    shared::initialize().await;

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(request_context_middleware))
            .configure(|config| shared::configure(config))
            .configure(|config| todos::configure(config, shared::get_db_connection()))
    })
    .bind(format!("0.0.0.0:{}", Env::instance().app_port))?
    .run()
    .await
}
