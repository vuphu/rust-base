use actix_http::Request;
use actix_web::dev::ServiceResponse;
use actix_web::{App, test};
use todos::configure;
use tokio::sync::OnceCell;

static INITIALIZER: OnceCell<()> = OnceCell::const_new();

async fn initialize()
-> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    INITIALIZER
        .get_or_init(|| async {
            shared::initialize().await;
        })
        .await;

    test::init_service(App::new().configure(|cfg| configure(cfg, shared::get_db_connection())))
        .await
}

pub mod todo_controller;
