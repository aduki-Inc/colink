use actix_web::{web, Scope};
use crate::handlers::org::org_handlers::*;

pub fn orgs_config() -> Scope {
  web::scope("/api/v1/org")
  
    // Institution/Organization routes
    .route("/create", web::put().to(create_institution))
}
