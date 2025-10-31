use redis::{Commands, Connection};

use crate::services::helpers::encode_nonce;
use crate::errors::bd_errors::CacheError;

pub fn get_redis_connection(client_url: &str) -> redis::RedisResult<Connection> {
    let client = redis::Client::open(client_url).unwrap();
    let con = client.get_connection()?;
    Ok(con)
}

pub fn put_nonce_into_cache(
    nonce: &str,
    pubkey: &str,
    client_url: &str,
) -> Result<(), CacheError> {
    let mut con = get_redis_connection(client_url)
        .map_err(CacheError::ConnectionError)?;

    con.hset_multiple::<_, _, _, ()>(encode_nonce(nonce), &[("address", pubkey), ("flag", "false")])
        .map_err(|e| CacheError::WriteError(e.to_string()))?;

    con.expire::<_, ()>(encode_nonce(nonce), 60)
        .map_err(|e| CacheError::WriteError(e.to_string()))?;

    Ok(())
}

pub fn check_nonce_in_cache(nonce: &str, pubkey: &str, client_url: &str) -> redis::RedisResult<bool> {
    let mut con = get_redis_connection(client_url)?;

    let vals: Vec<Option<String>> = con.hget(encode_nonce(nonce), &["address", "flag"])?;

    if let [Some(stored_pubkey), Some(flag)] = &vals[..] {
        Ok(stored_pubkey == pubkey && flag == "false")
    } else {
        Ok(false)
    }
}

pub fn reverse_flag(nonce: &str, client_url: &str) -> redis::RedisResult<()> {
    let mut con = get_redis_connection(client_url)?;

    let _: usize = con.hset(encode_nonce(nonce), "flag", "true")?;

    Ok(())
}