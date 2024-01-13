use diesel::pg::PgConnection;
use diesel::prelude::*;

// Establish database connection
pub async fn establish_connection(database_url: &String) -> PgConnection{

  PgConnection::establish(database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}