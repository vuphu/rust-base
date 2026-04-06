use crate::presentation::extractors::request_context::RequestContext;
use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

pub async fn request_context_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let locale = req
        .headers()
        .get("X-App-Language")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "en".to_string());

    rust_i18n::set_locale(&locale);
    req.extensions_mut().insert(RequestContext { locale });

    next.call(req).await
}
