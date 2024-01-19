use crate::db::schema::belongs::dsl::*;
// use crate::db::schema::belongs;
use crate::models::orgs::{Belong, EditBelong};
// use crate::models::custom_types::{RoleType, OrgType};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
// use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use crate::middlewares::auth::role_middleware::role_belong_deleted;
// use serde_json::json;

// Updating the Org member/Belong
pub fn belong_edited(belong_data: &EditBelong, conn: &mut PgConnection) -> Result<Belong, Error> {
  match diesel::update(belongs.filter(id.eq(belong_data.id).and(active.eq(true))))
  .set((
    identity.eq(&belong_data.identity),
    name.eq(&belong_data.name),
    title.eq(&belong_data.title)
  ))
  .get_result::<Belong>(conn) {
    Ok(belong) => Ok(belong),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}


// Updating the Org member/Belong - Staff status
pub fn belong_staff_edited(belong_id: &i32, staff_status: &bool, conn: &mut PgConnection) -> Result<Belong, Error> {
  match diesel::update(belongs.filter(id.eq(belong_id).and(active.eq(true))))
  .set(staff.eq(staff_status))
  .get_result::<Belong>(conn) {
    Ok(belong) => Ok(belong),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}

// Updating the Org member/Belong - Remove member
pub fn member_removed(role_author: &i32, role_section: &i32, belong_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  conn.transaction(|conn| {

    // First delete role associated with the member
    match role_belong_deleted(role_author, role_section, conn){
      Ok(true) => {
        match diesel::update(belongs.filter(id.eq(belong_id)))
        .set(active.eq(false))
        .execute(conn) {
          Ok(_) => Ok(true),
          Err(Error::NotFound) => Err(Error::NotFound),
          Err(err) => Err(err)
        }
      }
      Ok(false) => Ok(false),
      Err(err) => Err(err)
    }
  })
}

