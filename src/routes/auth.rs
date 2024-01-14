use actix_web::{web, Scope};
use crate::handlers::{ auth_handlers::* , role_handlers::* };

pub fn auth_config() -> Scope {
  web::scope("/api/v1")
    .route("/auth/register", web::post().to(register_user))
    .route("/auth/login", web::post().to(login_user))
    .route("/auth/check/username", web::post().to(check_username))
    .route("/auth/check", web::post().to(check_user))

    // Sections
    .route("/section/add", web::post().to(create_section))
}
