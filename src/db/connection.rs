use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::configs::config::Config;

// Establish database connection
pub async fn establish_connection() -> PgConnection{
  let config = Config::init();

  PgConnection::establish(&config.database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url))
}