use actix_web::{ web, HttpResponse, Error, post, get };
use serde_json::json;
use sqlx::PgPool;

use crate::{
    models::request_data::RegisterUserRequest, repositories::{
        get_user_by_public_key, 
        set_user
    }, services::helpers::serialize_uzer
};

// Endpoint to register a new user
#[post("/register")]
pub async fn register_user(
    pool: web::Data<PgPool>,
    user_info: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, Error> {
    let result = set_user(pool.clone(), user_info.public_key.clone()).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Ok()
                .json(json!(format!("User {} registered successfully", user_info.public_key))))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(json!(format!("Failed to register user, Error: {}", e))))
        }
    }
}

/// Endpoint to get user info
#[get("/{public_key}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    public_key: web::Path<String>,
) -> Result<HttpResponse, Error> {
    match get_user_by_public_key(pool.clone(), public_key.to_string()).await {
        
        Ok(user) => Ok(HttpResponse::Ok().json(
            serialize_uzer(user)
        )),
        
        Err(sqlx::Error::RowNotFound) => Ok(HttpResponse::NotFound().json(json!({
            "error": "User not found"
        }))),
        
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {}", e)
        }))),
    }
}