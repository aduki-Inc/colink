use actix_web::{web, App, HttpServer};
use std::sync::Arc;
mod routes;
mod models;
mod db;
mod controllers;
mod middlewares;
mod configs;
use configs::app_state::AppState;
use configs::config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let config = Config::init();

  // Create the AppState with the configuration
  let app_state = AppState::new(config);

  HttpServer::new(|| {
    App::new()
      .app_data(Arc::new(app_state))
      .service(web::resource("/").to(|| async { "hello world" }))
			.service(routes::auth::user())
			.service(routes::auth::auth())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}