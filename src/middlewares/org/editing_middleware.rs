use crate::db::schema::orgs::dsl::*;
// use crate::db::schema::roles::dsl::*;
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

//Creating the Organization
pub fn belongs_edited(user_id: &i32, user_name: &str, new_org: &InsertableOrganization, conn: &mut PgConnection) -> Result<Belong, Error> {
  match diesel::insert_into(approvals::table).values(&new_approval)
  .execute(conn) {
    Ok(_) => Ok(org),
    Err(err) => Err(err)
  }
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

    // Check if similar role exists?
    match diesel::insert_into(belongs::table).values(data)
    .get_result::<Belong>(conn) {
      Ok(belong) => {
        let roles_json = json!({
          "project": ["create", "read", "delete", "update"]
        });
        
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