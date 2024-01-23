use crate::db::schema::belongs::dsl::*;
// use crate::db::schema::belongs;
use crate::models::orgs::{Belong, EditBelong};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// Updating the Org member/Belong
pub fn belong_edited(belong_data: &EditBelong, conn: &mut PgConnection) -> Result<Belong, Error> {
  match diesel::update(belongs.filter(author.eq(belong_data.author).and(section.eq(belong_data.section)).and(active.eq(true))))
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
pub fn belong_staff_edited(author_id: &i32, section_id: &i32, staff_status: &bool, conn: &mut PgConnection) -> Result<Belong, Error> {
  match diesel::update(belongs.filter(author.eq(author_id).and(section.eq(section_id)).and(active.eq(true))))
  .set(staff.eq(staff_status))
  .get_result::<Belong>(conn) {
    Ok(belong) => Ok(belong),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}

// Updating the Org member/Belong - Remove member
pub fn member_disabled(author_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<Belong, Error> {
  // Update belong data to be false
  match diesel::update(belongs.filter(author.eq(author_id).and(section.eq(section_id))))
  .set(active.eq(false))
  .get_result::<Belong>(conn) {
    Ok(belong) => Ok(belong),
    Err(err) => Err(err)
  }
}


// Updating the Org member/Belong - Remove member
pub fn member_enabled(author_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<Belong, Error> {
  // Update belong data to be false
  match diesel::update(belongs.filter(author.eq(author_id).and(section.eq(section_id))))
  .set(active.eq(true))
  .get_result::<Belong>(conn) {
    Ok(belong) => Ok(belong),
    Err(err) => Err(err)
  }
}

// Check if the user is an active member of the organization
pub fn is_member_active(user_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  match belongs.filter(author.eq(user_id).and(section.eq(section_id)).and(active.eq(true))).first::<Belong>(conn) {
    Ok(_belong) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}
