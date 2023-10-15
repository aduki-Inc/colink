use crate::db::schema::users::dsl::*;
use crate::models::users::User;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

pub fn email_exists(other_email: &str, conn: &mut PgConnection) -> bool {
  match users.filter(email.eq(other_email)).first::<User>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}

pub fn username_exists(other_username: &str, conn: &mut PgConnection) -> bool {
  match users.filter(username.eq(other_username)).first::<User>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}