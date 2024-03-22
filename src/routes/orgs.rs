use actix_web::{web, Scope};
use crate::handlers::org::{
  org::*, member::*, info::*
};

pub fn org_config() -> Scope {
  web::scope("/api/v1/org")
    // Organization
    .route("/create", web::put().to(create_org))
    .route("/{org}/edit/logo", web::patch().to(update_logo))
    .route("/{org}/edit/background", web::patch().to(update_background))
    .route("/{org}/edit/info", web::patch().to(update_info))
    .route("/{org}/edit/contact", web::patch().to(update_contact))

    //Organization - Members
    .route("/{org}/users/add", web::put().to(add_user))
    .route("/{org}/users/edit", web::patch().to(edit_user_info))
    .route("/{org}/users/edit/status", web::patch().to(edit_staff_status))
    .route("/{org}/users/disable", web::patch().to(disable_user))
    .route("/{org}/users/enable", web::patch().to(enable_user))
}
