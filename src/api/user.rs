use actix_web::{ web, HttpResponse, Error, post, get };
use serde_json::json;
use sqlx::PgPool;

use crate::{
    models::request_data::RegisterUserRequest, 
    repositories::{
        get_user_by_public_key, 
        set_user
    }
};

// Endpoint to register a new user
#[post("/register")]
pub async fn register_user(
    pool: web::Data<PgPool>,
    user_info: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, Error> {
    let result = set_user(pool.clone(), user_info.clone()).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Ok()
                .json(json!(format!("User {} registered successfully", user_info.username))))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(json!(format!("Failed to register user, Error: {}", e))))
        }
    }
}

// Endpoint to get user info
#[get("/user/{public_key}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    public_key: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let response_from_db = get_user_by_public_key(pool.clone(), public_key.into_inner()).await;
    match response_from_db {
        Ok(user) => {
            return Ok(HttpResponse::Ok().json(user));
        },
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!(format!("Error retrieving user: {}", e))));
        }
    }
}