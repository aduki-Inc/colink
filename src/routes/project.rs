use actix_web::{web, Scope};
use crate::handlers::project::template::*;

pub fn template_config() -> Scope {
  web::scope("/api/v1/templates")

    // Templates Routes
    .route("/create", web::put().to(create_template))
}
