mod services;
mod models;
mod api;
mod middleware;
mod db_hooks;
mod errors;

use dotenv::dotenv;
use std::{ env };

use actix_web::{
    get,
    post,
    cookie::{ Cookie, time::Duration as CookieDuration, SameSite },
    web,
    App,
    Error,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_cors::Cors;
use models::{ request_data::{ AuthPayload, ForNonce }, config };
use serde_json::{ self, json };
use db_hooks::put_nonce_into_cache;

use crate::{api::{ authentification_hendler, send_nonce_hendler }, middleware::jwt_middleware::JwtMiddlewareFactory};

#[post("/authentication")]
pub async fn authentication(
    payload: web::Json<AuthPayload>,
    cfg: web::Data<config::Config>
) -> Result<HttpResponse, Error> {
    match authentification_hendler(&payload, cfg) {
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

            Ok(
                HttpResponse::Ok()
                    .cookie(access_cookie)
                    .cookie(refresh_cookie)
                    .json(json!({ "status": "ok" }))
            )
        }

        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("Error: {}", e))),
    }
}

#[get("/nonce")]
async fn send_nonce(payload: web::Json<ForNonce>, cfg: web::Data<config::Config>) -> impl Responder {
    let nonce = send_nonce_hendler();

    if let Err(e) = put_nonce_into_cache(&nonce, &payload.pubkey, &cfg.redis_url) {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "error": "Failed to store nonce: ".to_string() + &e.to_string()
        })
        );
    }

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
