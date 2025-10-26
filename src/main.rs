mod services;
mod models;
mod api;
mod middleware;

use actix_web::{ get, post, cookie::{Cookie, time::Duration as CookieDuration, SameSite}, web, App, Error, HttpResponse, HttpServer, Responder };
use actix_web::middleware::from_fn;
use actix_cors::Cors;
use models::request_data::AuthPayload;
use serde_json::{self, json};
use middleware::jwt_middleware_with_refresh;

use crate::api::{ authentification_hendler, send_nonce_hendler };

#[post("/authentication")]
pub async fn authentication(payload: web::Json<AuthPayload>) -> Result<HttpResponse, Error> {
    match authentification_hendler(&payload) {
        Ok((access_token, refresh_token)) => {
            let access_cookie = Cookie::build("access_token", access_token)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::None)
                .max_age(CookieDuration::hours(1))
                .path("/")
                .finish();

            let refresh_cookie = Cookie::build("refresh_token", refresh_token)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::None)
                .max_age(CookieDuration::days(7))
                .path("/")
                .finish();

            Ok(HttpResponse::Ok()
                .cookie(access_cookie)
                .cookie(refresh_cookie)
                .json(json!({ "status": "ok" })))
        }

        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("Error: {}", e))),
    }
}

#[get("/nonce")]
async fn send_nonce() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "nonce": send_nonce_hendler()
    }))
}

#[get("/check")]
async fn check_protection() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "protected"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization", "solana-client"])
                    .supports_credentials()
            )
            .service(authentication)
            .service(send_nonce)
            .service(
                web::scope("/protect")
                    .wrap(from_fn(jwt_middleware_with_refresh))
                    .service(check_protection)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}