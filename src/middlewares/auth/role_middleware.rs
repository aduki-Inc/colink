use crate::db::schema::sections::dsl::*;
use crate::db::schema::roles;
use crate::models::system::{Section, Role, NewRole};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;


pub fn role_exists(other_type: &str, section_id: &i32, user_id: i32, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(type_.eq(other_type).and(section.eq(section_id)).and(author.eq(user_id))).first::<Role>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(_) => Err(_),
  }
}

pub fn role_deleted(other_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {

  match diesel::delete(roles.filter(id.eq(other_id))).execute(conn) {
    Ok(1) => Ok(true),
    Ok(0) => Ok(false),
    Err(err) => Err(err),
    Ok(_) => Ok(false)
  }
}

pub fn role_updated(other_id: &i32, new_data: &Role, conn: &mut PgConnection) -> Result<Role, Error> {

  match diesel::update(role.filter(id.eq(other_id)))
  .set((
    name.eq(&new_data.name),
    type_.eq(&new_data.type_),
    privileges.eq(&new_data.privileges)
  ))
  .get_result(conn) {
    Ok(role) => Ok(role),
    Err(err) => Err(err)
  }
}