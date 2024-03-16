use crate::db::project::project::projects::dsl::*;
use crate::db::project::project::projects;
use crate::models::project::{
  Project, NewProject, InsertableProject
};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// Middleware to check if similar name already exists
pub async fn project_name_exists(other_name: &str, conn: &mut PgConnection) -> (bool) {
  match projects.filter(name.eq(other_name)).first::<Project>(conn) {
    Ok(project) => OK(true),
    Err(_) => Ok(false),
  }
}

//Middleware for creating new project
pub fn project_created(user_id: &i32, project: NewProject, conn: &mut PgConnection) -> Result<Project, Error> {
  let insertable_project = InsertableProject {
    author: *user_id,
    template: project.template,
    name: project.name,
    title: project.title,
    field: project.field,
    type_: project.type_,
    public: project.public,
    active: project.active,
    owned: project.owned,
    org: project.org,
    description: project.description,
  };
  match diesel::insert_into(projects::table).values(&insertable_project)
  .get_result::<Project>(conn) {
    Ok(project) => Ok(project),
    Err(err) => Err(err)
  }
}