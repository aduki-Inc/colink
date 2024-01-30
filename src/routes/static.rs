use actix_web::{web, Scope};
use crate::utils::file_util::index;

pub fn static_config() -> Scope {
  web::scope("")
    // Static file index route
    .route("/{filename:.*}", web::get().to(index))
}
