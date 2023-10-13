use actix_web::{web, Scope, HttpResponse};

pub fn user() -> Scope {
  web::scope("/user")
    .route("/get_all", web::get().to(|| async { HttpResponse::Ok().body("Get all users") }))
    .route("/get_by_id/:id", web::get().to(|| async { HttpResponse::Ok().body("Get user by ID") }))
}

pub fn auth() -> Scope {
  web::scope("/auth")
    .route("/login", web::post().to(|| async { HttpResponse::Ok().body("Login") }))
    .route("/logout", web::get().to(|| async { HttpResponse::Ok().body("Logout") }))
}
