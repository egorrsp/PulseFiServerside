mod services;
mod models;
mod api;
mod middleware;

use actix_web::{get, middleware::from_fn, post, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::request_data::AuthPayload;
use serde_json;
use middleware::jwt_middleware;


use crate::api::{authentification_hendler, send_nonce_hendler};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

#[post("/authentification")]
pub async fn authentification(payload: web::Json<AuthPayload>) -> Result<HttpResponse, Error> {
    match authentification_hendler(&payload) {
        Ok((access, refresh)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "access_token": access,
            "refresh_token": refresh
        }))),
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
    HttpResponse::Ok().body("Good")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .supports_credentials()
        )
            .service(hello)
            .service(authentification)
            .service(send_nonce)

            .service(
                web::scope("/protect")
                    .wrap(from_fn(jwt_middleware))
                    .service(check_protection)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}