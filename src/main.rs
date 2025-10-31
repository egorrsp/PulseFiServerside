mod services;
mod models;
mod api;
mod middleware;
mod db_hooks;
mod errors;

use dotenv::dotenv;
use std::{ env };

use actix_web::{
    post,
    get,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_cors::Cors;
use models::{ request_data::{ ForNonce }, config };
use serde_json::{ self };
use db_hooks::put_nonce_into_cache;

use crate::{
    api::{ authentication, logout, send_nonce_hendler },
    middleware::jwt_middleware::JwtMiddlewareFactory,
};

#[post("/nonce")]
async fn send_nonce(
    payload: web::Json<ForNonce>,
    cfg: web::Data<config::Config>
) -> impl Responder {
    let nonce = send_nonce_hendler();

    if let Err(e) = put_nonce_into_cache(&nonce, &payload.pubkey, &cfg.redis_url) {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "error": "Failed to store nonce: ".to_string() + &e.to_string()
        })
        );
    }

    HttpResponse::Ok().json(serde_json::json!({
        "nonce": nonce
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
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("check env file for REDIS_URL");
    let jwt_secret = env::var("JWT_SECRET").expect("check env file for JWT_SECRET");
    let bind_host = env::var("BIND_HOST").expect("check env file for BIND_HOST");
    let bind_port = env
        ::var("BIND_PORT")
        .unwrap()
        .parse::<u16>()
        .expect("check env file for BIND_PORT");

    let cfg = config::Config { redis_url, jwt_secret };
    let cfg = web::Data::new(cfg);

    HttpServer::new(move || {
        App::new()
            .app_data(cfg.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization", "solana-client"])
                    .supports_credentials()
            )
            .service(authentication)
            .service(send_nonce)
            .service(logout)
            .service(
                web
                    ::scope("/protect")
                    .wrap(JwtMiddlewareFactory {
                        jwt_secret: cfg.jwt_secret.clone(),
                    })
                    .service(check_protection)
            )
    })
        .bind((bind_host.to_string(), bind_port))?
        .run().await
}
