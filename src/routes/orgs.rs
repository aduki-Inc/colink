use actix_web::{web, Scope};
use crate::handlers::org::{
  org_handlers::*, member_handlers::*, update_handlers::*
};

pub fn org_config() -> Scope {
  web::scope("/api/v1/org")
  
    // Organization
    .route("/create", web::put().to(create_org))
    .route("/{org}/edit/logo", web::patch().to(update_logo))

    //Organization - Members
    .route("/{org}/users/add", web::put().to(add_member))
    .route("/{org}/users/edit", web::patch().to(edit_user_info))
    .route("/{org}/users/edit/status", web::patch().to(edit_staff_status))
    .route("/{org}/users/disable", web::patch().to(disable_user))
    .route("/{org}/users/enable", web::patch().to(enable_user))

}
