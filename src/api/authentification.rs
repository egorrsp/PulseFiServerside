use actix_web::web;
use bs58;
use crate::{
    repositories::put_nonce_into_cache, 
    models::request_data::{AuthPayload, ForNonce}, 
    services::{ check_signer, generate_nonce, generate_tokens }
};
use crate::repositories;
use crate::models::config;
use actix_web::{ post, get, HttpResponse, Error, Responder };
use actix_web::cookie::{ Cookie, time::Duration as CookieDuration, SameSite };
use serde_json::json;


// Endpoint to request a nonce
#[post("/nonce")]
async fn send_nonce(
    payload: web::Json<ForNonce>,
    cfg: web::Data<config::Config>
) -> impl Responder {
    let nonce = generate_nonce();

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


// Helper for fn up down below
pub fn authentification_hendler(
    payload: &web::Json<AuthPayload>,
    cfg: web::Data<config::Config>
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_key_vec = bs58::decode(&payload.public_key).into_vec()?;
    let public_key_bytes: [u8; 32] = public_key_vec
        .try_into()
        .map_err(|_| "Invalid public key length")?;

    let signature_vec = bs58::decode(&payload.signature).into_vec()?;
    let signature_bytes: [u8; 64] = signature_vec
        .try_into()
        .map_err(|_| "Invalid signature length")?;

    match repositories::check_nonce_in_cache(&payload.nonce, &payload.public_key, &cfg.redis_url) {
        Ok(true) => (),
        Err(_) | Ok(false) => return Err("Nonce not found or does not match".into()),
    }
    let valid = check_signer(&payload.nonce, &public_key_bytes, &signature_bytes)?;
    
    repositories::reverse_flag(&payload.nonce, &cfg.redis_url)?;

    if valid {
        Ok(generate_tokens(&payload.public_key, &cfg.jwt_secret))
    } else {
        Err("Invalid signature".into())
    }
}


// Endpoint for authentication
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


// Endpoint for logout
#[get("/logout")]
async fn logout() -> impl Responder {
    let access_cookie = Cookie::build("access_token", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(CookieDuration::seconds(0))
        .path("/")
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(CookieDuration::seconds(0))
        .path("/")
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(serde_json::json!({
            "status": "logged out"
        }))
}


// Protected endpoint example
#[get("/check")]
async fn check_protection() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "protected"
    }))
}