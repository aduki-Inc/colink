use actix_web::{web, Scope};
use crate::handlers::project::{template::*, project::*};

pub fn project_config() -> Scope {
  web::scope("/api/v1")

    //Project Routes
    .route("/projects/create", web::put().to(create_project))

    // Templates Routes
    .route("/templates/create", web::put().to(create_template))
    .route("/templates/{id}/edit", web::patch().to(update_template))
}
