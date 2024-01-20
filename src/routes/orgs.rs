use actix_web::{web, Scope};
use crate::handlers::org::{
  org_handlers::*, member_handlers::*
};

pub fn org_config() -> Scope {
  web::scope("/api/v1/org")
  
    // Organization
    .route("/create", web::put().to(create_org))

    //Organization - Members
    .route("/member/add", web::put().to(add_member))
    .route("/member/edit", web::patch().to(edit_member))
    .route("/member/edit/status", web::patch().to(edit_staff_status))
    .route("/member/disable", web::patch().to(disable_member))
    .route("/member/enable", web::patch().to(enable_member))
}
