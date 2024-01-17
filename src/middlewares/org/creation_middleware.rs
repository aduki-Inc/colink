use crate::db::schema::orgs::dsl::*;
use crate::db::schema::roles::dsl::roles;
use crate::db::schema::approvals::dsl::approvals;
use crate::db::schema::sections::dsl::sections;
use crate::models::{orgs::{Organization, InsertableOrganization}, system::{InsertableRole, NewSection, Section, InsertableApproval}};
use crate::models::custom_types::{RoleType, OrgType};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
// use chrono::{Utc, Duration};

pub fn org_exists(unique_name: &str, inst_name: &str, conn: &mut PgConnection) -> Result<bool, Error> {
  match orgs.filter(short_name.eq(unique_name).and(name.eq(inst_name))).first::<Organization>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}

//Creating the Organization
pub fn org_created(user_id: &i32, new_org: &InsertableOrganization, conn: &mut PgConnection) -> Result<Organization, Error> {
  conn.transaction(|conn| {
    match diesel::insert_into(orgs::table()).values(new_org)
    .get_result::<Organization>(conn) {
        Ok(org) => {
          let new_section = NewSection {
            name: org.short_name.clone(),
            target_id: org.id.clone(),
            target_name: org.name.clone()
          };

          match diesel::insert_into(sections::table()).values(&new_section)
          .get_result::<Section>(conn) {
              Ok(section) => {
                let new_role = InsertableRole {
                  section: section.id,
                  base: RoleType::Owner,
                  author: *user_id,
                  name: "Creator".to_owned(),
                  privileges: None,
                  expiry: None
                };

                match diesel::insert_into(roles::table()).values(&new_role)
                .execute(conn) {
                  Ok(_) => {
                    let org_name: String = match org.base {
                      OrgType::Ist => "Institution".to_owned(),
                      OrgType::Org => "Organization".to_owned(),
                      _=> "Organization".to_owned()
                    };
                    let new_approval = InsertableApproval {
                      target: org.id,
                      name: "org".to_owned(),
                      approved: Some(false),
                      description: Some(format!("Request to create an {}: {}", &org_name, &org.name))
                    };

                    match diesel::insert_into(approvals::table()).values(&new_approval)
                    .execute(conn) {
                      Ok(_) => Ok(org),
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
