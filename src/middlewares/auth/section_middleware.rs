use crate::db::schema::sections::dsl::*;
// use crate::db::schema::sections;
use crate::models::system::Section;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;


pub fn section_exists(other_identity: &str, conn: &mut PgConnection) -> bool {
  match sections.filter(identity.eq(other_identity)).first::<Section>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}

pub fn section_deleted(other_id: &i32, other_identity: &str, conn: &mut PgConnection) -> Result<bool, Error> {

  match diesel::delete(sections.filter(id.eq(other_id).and(identity.eq(other_identity)))).execute(conn) {
    Ok(1) => Ok(true),
    Ok(0) => Ok(false),
    Err(err) => Err(err),
    Ok(_) => Ok(false)
  }
}

pub fn section_updated(other_id: &i32, new_data: &Section, conn: &mut PgConnection) -> Result<Section, Error> {

  match diesel::update(sections.filter(id.eq(other_id)))
  .set((
    identity.eq(&new_data.identity),
    target.eq(&new_data.target),
    name.eq(&new_data.name),
    description.eq(&new_data.description)
  ))
  .get_result(conn) {
    Ok(section) => Ok(section),
    Err(err) => Err(err)
  }
}