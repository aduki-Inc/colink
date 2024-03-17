use crate::db::project::project::projects::dsl::*;
use crate::db::project::project::projects;
use crate::models::project::{
  Project, NewProject, InsertableProject
};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// Middleware to check if similar name already exists
// pub async fn project_name_exists(other_name: &str, conn: &mut PgConnection) -> bool {
//   match projects.filter(name.eq(other_name)).first::<Project>(conn) {
//     Ok(_) => true,
//     Err(_) => false,
//   }
// }

//Middleware for creating new project
pub async fn project_created(user_id: &i32, project: NewProject, conn: &mut PgConnection) -> Result<Project, Error> {
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
  conn.transaction(|conn|{
    match projects.filter(name.eq(&insertable_project.name)).first::<Project>(conn) {
      Ok(_) => {
        let new_err = Error::QueryBuilderError("Similar name already exits!".into());
        return Err(new_err);
      },
      Err(_) => {
        match diesel::insert_into(projects::table).values(&insertable_project)
        .get_result::<Project>(conn) {
          Ok(inserted_project) => Ok(inserted_project),
          Err(_) => {
            let err = Error::QueryBuilderError("Something went wrong, try again".into());
            return Err(err);
          }
        }
      },
    }
  })
}