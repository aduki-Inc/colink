use actix_web::{web, Scope};
use crate::handlers::auth::{ auth_handlers::*, section_handlers::*, role_handlers::* };

pub fn auth_config() -> Scope {
  web::scope("/api/v1")
    .route("/auth/register", web::post().to(register_user))
    .route("/auth/login", web::post().to(login_user))
    .route("/auth/check/username", web::post().to(check_username))
    .route("/auth/check", web::post().to(check_user))

    // Sections
    .route("/section/add", web::put().to(create_section))
    .route("/section/remove", web::delete().to(delete_section))
    .route("/section/edit", web::patch().to(update_section))

    //Roles
    .route("/role/add", web::put().to(create_role))
    .route("/role/remove", web::delete().to(delete_role))
    .route("/role/edit/privileges", web::patch().to(update_privileges))
    .route("/role/edit/expiry", web::patch().to(update_expiry))
}
