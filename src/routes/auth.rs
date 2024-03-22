use actix_web::{web, Scope};
use crate::handlers::auth::{ user::*, section::*, role::* };

pub fn auth_config() -> Scope {
  web::scope("/api/v1/auth")
    .route("/register", web::post().to(register_user))
    .route("/login", web::post().to(login_user))
    .route("/check/username", web::post().to(check_username))
    .route("/check", web::post().to(check_user))

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
