use actix_web::web;
use sqlx::PgPool;

use crate::models::logs_bd_struct::NewLogRecord;

pub async fn create_log_helper(
    db: web::Data<PgPool>,
    payload: NewLogRecord
) -> Result<bool, sqlx::error::Error> {
    let record = sqlx::query(
        r#"
        INSERT INTO logs (
            user_public_key, 
            source, 
            error_code, 
            message, 
            criticality, 
            context
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#)
        .bind(payload.user_public_key)
        .bind(payload.source)
        .bind(payload.error_code)
        .bind(payload.message)
        .bind(payload.criticality)
        .bind(payload.context)
        .execute(db.as_ref())
        .await;

    match record {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}