use actix_web::test::{TestRequest, call_service};
use chrono::Utc;
use fake::{Fake, faker::lorem::en as lorem};
use serde_json::json;

use crate::e2e::initialize;

#[cfg(test)]
mod get_todos {
    use super::*;

    #[actix_rt::test]
    async fn should_return_success() {
        let app = initialize().await;
        let request = TestRequest::get().uri("/todos").to_request();
        let response = call_service(&app, request).await;
        assert_eq!(response.status(), 200);
    }
}

#[cfg(test)]
mod create_todo {
    use super::*;

    #[actix_rt::test]
    async fn should_return_created() {
        let app = initialize().await;
        let request = TestRequest::post()
            .uri("/todos")
            .set_json(&json!({
                "title": lorem::Word().fake::<String>(),
                "due_date": (Utc::now() + chrono::Duration::days(1)).to_rfc3339()
            }))
            .to_request();
        let response = call_service(&app, request).await;
        assert_eq!(response.status(), 201);
    }

    #[actix_rt::test]
    async fn should_return_unprocessable_entity() {
        let app = initialize().await;
        let request = TestRequest::post().uri("/todos").set_json(&json!({})).to_request();
        let response = call_service(&app, request).await;
        assert_eq!(response.status(), 422);
    }
}
