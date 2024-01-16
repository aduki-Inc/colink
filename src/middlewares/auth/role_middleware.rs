use crate::db::schema::roles::dsl::*;
// use crate::db::schema::roles;
use crate::models::system::{Role, RolePrivileges, RoleExpiry };
use actix_web::web::Json;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use chrono::{Utc, Duration};


pub fn role_exists(user_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
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

pub fn privileges_updated(new_data: &RolePrivileges, conn: &mut PgConnection) -> Result<Role, Error> {

  // let updated_json_data = &new_data.privileges;
  // let update_query = format!(
  //   "UPDATE roles SET privileges = '{}'::json, base = '{}' WHERE id = {} RETURNING", 
  //   updated_json_data, &new_data.base, &new_data.id
  // );

  // match diesel::sql_query(update_query)
  // .get_result::<Role>(conn) {
  //   Ok(role) => Ok(role),
  //   Err(Error::NotFound) => Err(Error::NotFound),
  //   Err(err) => Err(err)
  // }

  match diesel::update(roles.filter(id.eq(new_data.id)))
  .set((
    base.eq(&new_data.base),
    privileges.eq(&new_data.privileges)
  ))
  .get_result(conn) {
    Ok(role) => Ok(role),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}


pub fn expiry_updated(data: &RoleExpiry, conn: &mut PgConnection) -> Result<Role, Error> {

  let duration = Duration::days(data.expiry);

  match roles.filter(id.eq(data.id)).first::<Role>(conn) {
    Ok(mut role) => {
      // If expiry days exists add the supplied number/ else supplied convert to future date from today
      if role.expiry.is_some() {
        let date_time = role.expiry.unwrap() + duration;
        role.expiry = Some(date_time);
      } else {
        let initial_date = Utc::now();

        let future_date = initial_date + duration;

        role.expiry = Some(future_date.naive_utc())
      };

      match diesel::update(roles.filter(id.eq(role.id)))
      .set(expiry.eq(role.expiry))
      .get_result(conn) {
        Ok(role) => Ok(role),
        Err(err) => Err(err)
      }
    },
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}