use crate::db::project::project::projects::dsl::*;
use crate::db::project::project::projects;
use crate::db::org::org::orgs::dsl::orgs;
use crate::db::org::org::orgs::short_name;
use crate::models::{
  project::{ Project, NewProject, InsertableProject},
  orgs::Organization
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
    name: project.name,
    title: project.title,
    field: project.field,
    public: project.public,
    active: true,
    owned: false,
    org: None,
  };
  conn.transaction(|conn|{
    match projects.filter(name.eq(&insertable_project.name).and(author.eq(user_id))).first::<Project>(conn) {
      Ok(_) => {
        return Err(Error::QueryBuilderError("You already have a project with similar name!".into()));
      },
      Err(_) => {
        match diesel::insert_into(projects::table).values(&insertable_project)
        .get_result::<Project>(conn) {
          Ok(inserted_project) => Ok(inserted_project),
          Err(_) => {
            return Err(Error::QueryBuilderError("Something went wrong, try again".into()));
          }
        }
      },
    }
  })
}

// Middleware for creating org project
pub async fn org_project_created(user_id: &i32, org_short_name: &str, project: NewProject, conn: &mut PgConnection) -> Result<Project, Error> {
  conn.transaction(|conn|{

    match orgs.filter(short_name.eq(org_short_name)).first::<Organization>(conn) {
      Ok(org_data) => {
        match projects.filter(name.eq(&project.name).and(org.eq(&org_data.id))).first::<Project>(conn) {
          Ok(_) => {
            return Err(Error::QueryBuilderError("Project with similar name already exits in this organization!".into()));
          },
          Err(Error::NotFound) => {
            let insertable_project = InsertableProject {
              author: *user_id,
              name: project.name,
              title: project.title,
              field: project.field,
              public: project.public,
              active: true,
              owned: true,
              org: Some(org_data.id),
            };

            match diesel::insert_into(projects::table).values(&insertable_project)
            .get_result::<Project>(conn) {
              Ok(inserted_project) => Ok(inserted_project),
              Err(_err) => {
                // println!("{:?}", err);
                return Err(Error::QueryBuilderError("Something went wrong, try again".into()));
              }
            }
          },
          Err(_err) => {
            // println!("{:?}", err);
            return Err(Error::QueryBuilderError("Something went wrong, try again".into()));
          }
        }
      },
      Err(_) => {
        return Err(Error::QueryBuilderError("The organization does't seem to exists!".into()));
      }
    }
  })
}