use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis connection failed: {0}")]
    ConnectionError(#[from] redis::RedisError),

    #[error("Failed to store nonce: {0}")]
    WriteError(String),
}