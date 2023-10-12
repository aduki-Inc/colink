use actix_web::{web, HttpResponse};

pub fn user() -> web::Scope {
    web::scope("/user")
        .route("/get_all", web::get().to(|| async { HttpResponse::Ok().body("Get all users") }))
        .route("/get_by_id/:id", web::get().to(|| async { HttpResponse::Ok().body("Get user by ID") }))
}

pub fn auth() -> web::Scope {
    web::scope("/auth")
        .route("/login", web::post().to(|| async { HttpResponse::Ok().body("Login") }))
        .route("/logout", web::get().to(|| async { HttpResponse::Ok().body("Logout") }))
}