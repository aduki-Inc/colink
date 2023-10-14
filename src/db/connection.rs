use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use config::DatabaseConfig;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error>{
  
  //Retrieve database connection details from config
  let db_config = DatabaseConfig::init();

  //Create a database connection options
  let db_options = PgConnectOptions::new()
    .database(&db_config.db_name)
    .username(&db_config.db_user)
    .password(&db_config.db_password)
    .host(&db_config.db_host)
    .port(&db_config.db_port);

  //Create a database connection pool
  let pool = PgPool::connect_with(db_options).await?;
  Ok(pool)
}