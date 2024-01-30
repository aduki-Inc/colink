use crate::db::schema::orgs::dsl::*;
//use crate::db::schema::belongs::dsl::*;
// use crate::db::schema::approvals::dsl::approvals;
// use crate::db::schema::sections::dsl::sections;
use crate::db::schema::{roles, orgs as org_model, approvals, sections, belongs};
use crate::models::{
  orgs::{
    Organization, InsertableOrganization, Belong,
    InsertableBelong, BelongIntermediate
  }, 
  system::{InsertableRole, NewSection, Section, InsertableApproval}
};
use crate::models::custom_types::{RoleType, OrgType};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde_json::json;


pub fn org_exists(unique_name: &str, other_name: &str, conn: &mut PgConnection) -> Result<bool, Error> {
  match orgs.filter(short_name.eq(unique_name).and(name.eq(other_name))).first::<Organization>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}

pub fn belong_exists(user_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  use crate::db::schema::belongs::dsl::*;
  match belongs.filter(author.eq(user_id).and(section.eq(section_id))).first::<Belong>(conn) {
    Ok(_role) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}


//Creating the Organization
pub fn org_created(user_id: &i32, user_name: &str, new_org: &InsertableOrganization, conn: &mut PgConnection) -> Result<Organization, Error> {
  conn.transaction(|conn| {
    match diesel::insert_into(org_model::table).values(new_org)
    .get_result::<Organization>(conn) {
        Ok(org) => {
          let new_section = NewSection {
            identity: org.short_name.clone(),
            target: org.id.clone(),
            name: org.short_name.clone(),
            description: Some(format!("Section for {}", &org.name))
          };

          match diesel::insert_into(sections::table).values(&new_section)
          .get_result::<Section>(conn) {
              Ok(inserted_section) => {
                let new_belong = InsertableBelong {
                  author: *user_id,
                  org: org.id.clone(),
                  section: inserted_section.id,
                  name: (*user_name).to_string(),
                  identity: "Creator".to_owned(),
                  title: "Creator".to_owned(),
                  staff: Some(true),
                };

                match diesel::insert_into(belongs::table).values(&new_belong)
                .execute(conn) {
                  Ok(_) => {
                    let roles_json = json!({
                      "project": ["create", "read", "update", "delete"],
                      "members": ["create", "read", "update", "delete"],
                      "staff": ["create", "read", "update", "delete"]
                    });
            
                    let new_role = InsertableRole {
                      section: inserted_section.id,
                      base: RoleType::Owner,
                      author: *user_id,
                      name: "Creator".to_owned(),
                      privileges: Some(roles_json),
                      expiry: None
                    };

                    // print!("{:?}", new_role);

                    match diesel::insert_into(roles::table).values(&new_role)
                    .execute(conn) {
                      Ok(_) => {
                        let org_name: String = match org.base {
                          OrgType::Ist => "Institution".to_owned(),
                          OrgType::Org => "Organization".to_owned()
                        };
                        let new_approval = InsertableApproval {
                          target: org.id,
                          name: "org".to_owned(),
                          approved: Some(false),
                          description: Some(format!("Request to create an {}: {}", &org_name, &org.name))
                        };

                        match diesel::insert_into(approvals::table).values(&new_approval)
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
        }
        Err(err) => Err(err)
    }
  })
}


// Creating belongs and role for users that belongs to a particular Org
pub fn belongs_created(inter_m: &BelongIntermediate, data: &InsertableBelong, conn: &mut PgConnection) -> Result<Belong, Error> {
  let role_type: RoleType = match data.staff.unwrap() {
    true => RoleType::Staff,
    false => RoleType::Period
  };

  let role_name: String = match role_type {
    RoleType::Owner => "Creator".to_owned(),
    RoleType::Admin => "Admin".to_owned(),
    RoleType::Staff => "Staff".to_owned(),
    RoleType::Period => "Member".to_owned()
  };

  let expiry_date: Option<NaiveDateTime> = match NaiveDate::parse_from_str(&inter_m.date.clone().unwrap(), "%Y-%m-%d"){
    Ok(created_date) => {
      let time = NaiveTime::from_hms_opt(0,0,0).unwrap();

      Some(NaiveDateTime::new(created_date, time))
    },
    Err(_) => None,
  };

  conn.transaction(|conn| {

    // Create new belong
    match diesel::insert_into(belongs::table).values(data)
    .get_result::<Belong>(conn) {
      Ok(belong) => {
        let roles_json = json!({
          "project": ["create", "read", "update", "delete"]
        });

        // print!("{}", roles_json);
        
        let new_role = InsertableRole {
          section: inter_m.section,
          base: role_type,
          author: data.author,
          name: role_name,
          privileges: Some(roles_json),
          expiry: expiry_date
        };

        //Create Role
        match diesel::insert_into(roles::table).values(&new_role)
        .execute(conn) {
          Ok(_) => Ok(belong),
          Err(err) => Err(err)
        }
      }
      Err(err) => Err(err)
    }
  })
}