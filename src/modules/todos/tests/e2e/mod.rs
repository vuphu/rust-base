use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{App, Error, test};
use todos::configure;

async fn initialize() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    shared::initialize().await;

    test::init_service(App::new().configure(|cfg| configure(cfg, shared::get_db_connection())))
        .await
}

pub mod todo_controller;
