use actix_web::{Responder, get, web};

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(index);
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = String)
    )
)]
#[get("/")]
async fn index() -> impl Responder {
    "Do not dwell in the past, do not dream of the future, concentrate the mind on the present moment. - Buddha"
}
