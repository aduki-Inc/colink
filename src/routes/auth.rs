use actix_web::{web, Scope};
use crate::handlers::{ auth_handlers::* , role_handlers::* };

pub fn auth_config() -> Scope {
  web::scope("/api/v1/auth")
    .route("/register", web::post().to(register_user))
    .route("/login", web::post().to(login_user))
    .route("/check/username", web::post().to(check_username))
    .route("/check", web::post().to(check_user));
  web::scope("/api/v1/section")
    .route("/add", web::post().to(create_section))
}
