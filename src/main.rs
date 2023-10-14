use actix_web::{web, App, HttpServer};
use diesel_migrations;
mod routes;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  diesel_migrations::run_pending_migrations(&db::connection).unwrap();

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