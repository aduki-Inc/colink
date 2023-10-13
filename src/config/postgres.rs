use dotenv::dotenv;
use std::env;

//Load environment variables from env
pub struct DatabaseConfig{
  pub db_name: String,
  pub db_user: String,
  pub db_password: String,
  pub db_host: String,
  pub db_port: String
}

impl DatabaseConfig {
  pub fn init() -> Self{
    dotenv.ok();

    let db_name = env::var("DB_NAME").expect("DB_NAME must be in the .env file");
    let db_user = env::var("DB_USER").expect("DB_USER must be in the .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be in the .env file");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be in the .env file");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be in the .env file")
      .parse()
      .expect("Invalid DB_PORT in the .env file");

    Self{
      db_name, db_user,
      db_password, db_host,
      db_port
    }
  }
}


// pub fn get_var(key: &str) -> String {
//   env::var(key).expect(&format!("{} must be set in .env file", key))
// }

// pub fn get_var_as_u16(key: &str) -> u16 {
//   env::var(key)
//     .expect(&format!("{} must be set in .env file", key))
//     .parse()
//     .expect(&format!("Invalid {} in .env file", key))
// }