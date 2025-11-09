use actix_web::{post, web, HttpResponse, Error};
use serde_json::json;
use sqlx::PgPool;

use crate::models::logs_bd_struct::NewLogRecord;
use crate::repositories::logs_bd::create_log_helper;


#[post("/logs")]
pub async fn record_log(
    pool: web::Data<PgPool>,
    payload: web::Json<NewLogRecord>
) -> Result<HttpResponse, Error> {

    let payload_open: NewLogRecord = payload.into_inner();

    let record = 
        create_log_helper(pool, payload_open.clone()).await;

    match record {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!(format!("log from user {} added succeful", &payload_open.user_public_key)))),
        Err(e) => Ok(HttpResponse::InternalServerError()
            .json(json!(format!("Log from {} didnt added", &payload_open.user_public_key))))
    }
}