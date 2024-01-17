mod configs;
mod db;
mod handlers;
mod middlewares;
mod models;
mod routes;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
extern crate diesel_derive_enum;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	let app_data = web::Data::new(
		configs::state::AppState { 
			counter: Mutex::new(0),
			config: configs::config::Config::init() 
		}
	);

	HttpServer::new(move || {
		let cors = Cors::default()
			.allowed_origin("http://localhost:8080")
			.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
			.allowed_headers(vec![
				header::CONTENT_TYPE,
				header::AUTHORIZATION,
				header::ACCEPT,
			])
			.max_age(3600)
			.supports_credentials();

		App::new()
			.app_data(app_data.clone())
			.app_data(web::JsonConfig::default()
				.limit(4096)
				.error_handler(|err, _req| handlers::error_handlers::json_cfg(err)),
			)
			.wrap(cors)
			.wrap(Logger::default())
			.service(routes::orgs::org_config())
			.service(routes::auth::auth_config())
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
