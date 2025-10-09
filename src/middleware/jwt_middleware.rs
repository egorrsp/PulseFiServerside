use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use actix_web_lab::middleware::from_fn;
use crate::utils::jwt::verify_token;

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: actix_web_lab::middleware::Next<ServiceRequest>,
) -> Result<ServiceResponse, Error> {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(value_str) = header_value.to_str() {
            if value_str.starts_with("Bearer ") {
                let token = value_str.trim_start_matches("Bearer ").trim();
                if verify_token(token) {
                    return next.call(req).await;
                }
            }
        }
    }

    Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
}