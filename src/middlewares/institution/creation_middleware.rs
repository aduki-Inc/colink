use crate::db::schema::roles::dsl::*;
use crate::db::schema::institutions::dsl::*;
use crate::db::schema::approvals::dsl::*;
use crate::db::schema::belongs::dsl::*;
use crate::db::schema::sections::dsl::*;
use crate::models::system::Approval;
// use crate::db::schema::roles;
use crate::models::{institutions::{Institution, Belong, InsertableInstitution}, system::{InsertableRole, NewSection, Section, InsertableApproval}};
use crate::models::custom_types::RoleType;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
// use chrono::{Utc, Duration};

pub fn institution_exists(unique_name: &str, inst_name: &str, conn: &mut PgConnection) -> Result<bool, Error> {
  match institutions.filter(short_name.eq(unique_name).or(name.eq(inst_name))).first::<Institution>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}

//Creating the institution
pub fn institution_created(user_id: &i32, &new_institution: InsertableInstitution, conn: &mut PgConnection) -> Result<Institution, Error> {
  conn.transaction(|conn| {
    match diesel::insert_into(institutions::table).values(new_institution)
    .get_result::<Institution>(conn) {
        Ok(institution) => {
          let new_section = NewSection {
            name: &institution.short_name,
            target_id: &institution.id,
            target_name: &institution.short_name
          };

          match diesel::insert_into(sections::table).values(&new_section)
          .get_result::<Section>(conn) {
              Ok(section) => {
                let new_role = InsertableRole {
                  section: &section.id,
                  base: RoleType::Owner,
                  author: user_id,
                  name: "Creator".to_str(),
                  privileges: None,
                  expiry: None
                };

                match diesel::insert_into(roles::table).values(&new_role)
                .execute(conn) {
                  Ok(_) => {
                    let new_approval = InsertableApproval {
                      target: &institution.id,
                      name: &institution.name,
                      approved: Some(false),
                      description: Some(format!("Request to create an institution: {}", &institution.name))
                    };

                    match diesel::insert_into(approvals::table).values(&new_approval)
                    .execute(conn) {
                      Ok(_) => Ok(institution),
                      Err(err) => Err(err)
                    }
                  }
                  Err(err) => Err(err)
                }
                
              }
              Err(err) => Err(err)
          }
        }
        Err(err) => Err(err)
    }
  })
}
