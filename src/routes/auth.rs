use actix_web::{web, Scope, HttpResponse};
use crate::controllers::auth_controller::register_user;

pub fn user() -> Scope {
  web::scope("/user")
    .route("/get_all", web::get().to(|| async { HttpResponse::Ok().body("Get all users") }))
    .route("/get_by_id/:id", web::get().to(|| async { HttpResponse::Ok().body("Get user by ID") }))
}

pub fn auth() -> Scope {
  web::scope("api/v1/auth")
    .route("/register", web::post().to(register_user))
    .route("/login", web::get().to(|| async { HttpResponse::Ok().body("Logout") }))
}
