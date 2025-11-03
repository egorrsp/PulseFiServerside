mod services;
mod models;
mod api;
mod middleware;
mod repositories;
mod errors;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{ env };
use actix_web::{
    web,
    App,
    HttpServer,
};
use actix_cors::Cors;
use models::config;
use crate::{
    api::{ authentication, check_protection, get_user, logout, register_user, send_nonce },
    middleware::jwt_middleware::JwtMiddlewareFactory,
};



// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let postgres_url = env::var("DATABASE_URL").expect("check env file for DATABASE_URL");
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .expect("Error in connecting db");

    HttpServer::new(move || {
        App::new()
            .app_data(cfg.clone())
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .supports_credentials()
            )
            .service(authentication)
            .service(send_nonce)
            .service(logout)
            .service(
                web
                    ::scope("/user")
                    .service(register_user)
                    .service(get_user)
            )
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
