use crate::db::schema::sections::dsl::*;
use crate::db::schema::sections;
use crate::models::system::{Section, NewSection};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;


pub fn section_exists(other_name: &str, conn: &mut PgConnection) -> bool {
  match sections.filter(name.eq(other_name)).first::<Section>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}

pub fn section_deleted(other_id: &i32, other_name: &str, conn: &mut PgConnection) -> Result<bool, Error> {

  match diesel::delete(sections.filter(id.eq(other_id).and(name.eq(other_name)))).execute(conn) {
    Ok(1) => Ok(true),
    Ok(0) => Ok(false),
    Err(err) => Err(err),
    Ok(_) => Ok(false)
  }
}

pub fn section_updated(other_id: &i32, new_data: &Section, conn: &mut PgConnection) -> Result<Section, Error> {

  match diesel::update(sections.filter(id.eq(other_id)))
  .set((
    name.eq(&new_data.name),
    target_id.eq(&new_data.target_id),
    target_name.eq(&new_data.target_name)
  ))
  .get_result(conn) {
    Ok(section) => Ok(section),
    Err(err) => Err(err)
  }
}