mod configs;
mod db;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utilities;

// use std::path::PathBuf;
use std::sync::Mutex;
use actix_files::Files;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, guard, web, App, HttpServer};
extern crate diesel_derive_enum;
extern crate tempdir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	// Get current directory of the app
	let static_path = match std::env::current_dir() {
    Ok(root_path) => {
			root_path.join("static").display().to_string()
		},
    Err(_) => {
			String::from("static")
		}
	};

	// println!("Root Dir: {:?}", static_path);
	let app_data = web::Data::new(
		configs::state::AppState {
			counter: Mutex::new(0),
			static_dir: static_path,
			config: configs::config::Config::init()
		}
	);

	HttpServer::new(move || {
		let cors = Cors::default()
			.allowed_origin("http://localhost:8080")
			.allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
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
				.error_handler(|err, _req| handlers::system::json_cfg(err)),
			)
			.wrap(cors)
			.service(Files::new("/static", "./static"))
			.default_service(
				web::route()
					.guard(guard::Not(guard::Get()))
					.to(handlers::system::url_not_found)
			)
			.wrap(Logger::default())
			.configure(routes::init)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}