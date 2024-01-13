mod configs;
mod db;
mod handlers;
mod middlewares;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{error, http::header, web, App, HttpResponse, HttpServer};
use serde_json::json;
use std::sync::Arc;
extern crate diesel_derive_enum;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            // create custom error response
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .app_data(json_cfg.clone())
            .service(routes::auth::auth_config())
            //			.wrap(handlers::error_handlers::MyErrorHandler)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
