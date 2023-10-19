use actix_web::{web, App, HttpServer};
extern crate diesel_derive_enum;
mod routes;
mod models;
mod db;
mod controllers;
mod middlewares;
mod configs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").to(|| async { "hello world" }))
			.service(routes::auth::user())
			.service(routes::auth::auth())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}