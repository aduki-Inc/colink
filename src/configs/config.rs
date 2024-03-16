use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
  pub jwt_secret: String,
  pub jwt_expires_in: u64,
  pub jwt_maxage: i32,
  pub git_path: PathBuf
}

impl Config {
  pub fn init() -> Config {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expires_in = env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
    let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
    let git_path = env::var("GIT_PATH").expect("GIT_PATH must be set");

    // Create path to for git objects
	  let path = PathBuf::from(&git_path);

    Config {
      database_url,
      jwt_secret,
      jwt_expires_in: jwt_expires_in.parse::<u64>().unwrap(),
      jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
      git_path: path
    }
  }
}