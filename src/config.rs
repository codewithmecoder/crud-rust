#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
  pub jwt_secret: String,
  pub jwt_maxage: i64,
  pub port: u16,
}

impl Config {
  pub fn init() -> Config{
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let jwt_maxage = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set").parse::<i64>().unwrap();

    Config {
      database_url,
      jwt_secret,
      jwt_maxage,
      port: 8000,
    }
  }
}