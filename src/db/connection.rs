use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
// use crate::configs::config::Config;

pub async fn establish_connection() -> PgConnection{
  dotenv().ok();

  // Get database url from env file
  let database_url = env::var("DATABASE_URL")
    .expect("Database url must be set in .env file");

  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}