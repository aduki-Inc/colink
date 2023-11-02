mod routes;
mod db;
mod handlers;
mod middlewares;
mod configs;
mod models;

use actix_web::{App, HttpServer, http::header};
use actix_web::middleware::Logger;
use actix_cors::Cors;
extern crate diesel_derive_enum;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new( move || {
    let cors = Cors::default()
      .allowed_origin("http://localhost:8080")
      .allowed_methods(vec!["GET", "POST", "PUT"])
      .allowed_headers(vec![
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        header::ACCEPT,
      ])
      .supports_credentials();
    App::new()
			.service(routes::auth::user())
			.service(routes::auth::auth())
      .wrap(cors)
      .wrap(Logger::default())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}