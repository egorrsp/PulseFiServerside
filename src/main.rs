mod services;
mod models;
mod api;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::request_data::AuthPayload;

use crate::api::{authentification_hendler, send_nonce_hendler};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

#[post("/authorise")]
async fn authentification(payload: web::Json<AuthPayload>) -> impl Responder {
    match authentification_hendler(&payload) {
        Ok(true) => HttpResponse::Ok().body("✅ Подпись верна"),
        Ok(false) => HttpResponse::Unauthorized().body("❌ Неверная подпись"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка: {}", e)),
    }
}

#[get("/nonce")]
async fn send_nonce() -> impl Responder {
    HttpResponse::Ok().body(send_nonce_hendler())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(authentification)
            .service(send_nonce)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}