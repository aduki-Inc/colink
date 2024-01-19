use actix_web::{web, Scope};
use crate::handlers::org::org_handlers::*;

pub fn org_config() -> Scope {
  web::scope("/api/v1/org")
  
    // Organization routes
    .route("/create", web::put().to(create_org))

    //Organization Members
    .route("/member/add", web::put().to(add_org_member))
}
