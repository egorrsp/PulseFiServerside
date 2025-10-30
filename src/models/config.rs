#[derive(Debug, Clone)]
pub struct Config {
    pub redis_url: String,
    pub jwt_secret: String,
}