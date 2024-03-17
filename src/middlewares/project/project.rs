use crate::db::project::project::projects::dsl::*;
use crate::db::project::project::projects;
use crate::db::org::org::orgs;
use crate::models::{
  project::{ Project, NewProject, InsertableProject}
  orgs::Organization
}

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
    name: project.name,
    title: project.title,
    field: project.field,
    public: project.public,
    active: true,
    owned: false,
    org: None,
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
            // println!("{:?}", error);
            let err = Error::QueryBuilderError("Something went wrong, try again".into());
            return Err(err);
          }
        }
      },
    }
  })
}

// Middleware for creating org project
pub async fn org_project_created(user_id: &i32, org_short_name: &str, project: NewProject, conn: &mut PgConnection) -> Result<Project, Error> {
  conn.transaction(|conn|{
    match projects.filter(name.eq(&insertable_project.name)).first::<Project>(conn) {
      Ok(_) => {
        return Error::QueryBuilderError("Similar name already exits!".into());
      },
      Err(_) => {
        match projects.filter(short_name.eq(org_short_name)).first::<Organization>(conn) {
          Ok(org) => {
            let insertable_project = InsertableProject {
              author: *user_id,
              name: project.name,
              title: project.title,
              field: project.field,
              public: project.public,
              active: true,
              owned: true,
              org: org.id,
            };
            match diesel::insert_into(projects::table).values(&insertable_project)
            .get_result::<Project>(conn) {
              Ok(inserted_project) => Ok(inserted_project),
              Err(_) => {
                // println!("{:?}", error);
                return Error::QueryBuilderError("Something went wrong, try again".into());
              }
            }
          }
          Err(_) => {
            return Error::QueryBuilderError("Organization does not seem to exists, try again".into());
          }
        }
      },
    }
  })
}