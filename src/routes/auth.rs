use actix_web::{web, Scope, HttpResponse};
use crate::handlers::auth_handlers::{register_user, login_user, check_user};

pub fn auth() -> Scope {
  web::scope("api/v1/auth")
    .route("/register", web::post().to(register_user))
    .route("/login", web::post().to(login_user))
    .route("/check", web::post().to(check_user))
}
