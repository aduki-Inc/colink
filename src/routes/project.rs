use actix_web::{web, Scope};
use crate::handlers::project::{template::*, project::*};

pub fn project_config() -> Scope {
  web::scope("/api/v1/project")

    //Project Routes
    .route("/add", web::put().to(create_project))
    .route("/{org}/add", web::put().to(create_org_project))
}

pub fn template_config() -> Scope {
  web::scope("/api/v1/template")
    // Templates Routes
    .route("/add", web::put().to(create_template))
    .route("/{id}/edit", web::patch().to(update_template))
}
