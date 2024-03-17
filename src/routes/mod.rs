use actix_web::web;
pub mod auth;
pub mod orgs;
pub mod r#static;
pub mod project;

// Fn to configure all services/scopes
pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(orgs::org_config());
  cfg.service(auth::auth_config());
  cfg.service(project::project_config());
  cfg.service(project::template_config());
  cfg.service(r#static::static_config());
}