use redis::{Commands, Connection};


pub fn get_redis_connection(client_url: &str) -> redis::RedisResult<Connection> {
    let client = redis::Client::open(client_url).unwrap();
    let con = client.get_connection()?;
    Ok(con)
}

pub fn put_nonce_into_cache(nonce: &str, pubkey: &str, client_url: &str) -> redis::RedisResult<()> {
    let mut con = get_redis_connection(client_url)?;

    let _: usize = con.hset_multiple(nonce, &[("address", pubkey), ("flag", &"false")])?;
    let _: usize = con.expire(nonce, 60)?;

    Ok(())
}

pub fn check_nonce_in_cache(nonce: &str, pubkey: &str, client_url: &str) -> redis::RedisResult<bool> {
    let mut con = get_redis_connection(client_url)?;

    let vals: Vec<Option<String>> = con.hget(nonce, &["address", "flag"])?;

    if let [Some(stored_pubkey), Some(flag)] = &vals[..] {
        Ok(stored_pubkey == pubkey && flag == "false")
    } else {
        Ok(false)
    }
}

pub fn reverse_flag(nonce: &str, client_url: &str) -> redis::RedisResult<()> {
    let mut con = get_redis_connection(client_url)?;

    let _: usize = con.hset(nonce, "flag", "true")?;

    Ok(())
}