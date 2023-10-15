use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

pub fn email_exists(other_email: &str, conn: &PgConnection) -> bool {
    match users.filter(email.eq(other_email)).first::<(i32, String, String, String)>(conn) {
        Ok(_) => true,
        Err(Error::NotFound) => false,
        Err(_) => false,
    }
}

pub fn username_exists(username: &str, conn: &PgConnection) -> bool {
    match users.filter(username.eq(username)).first::<(i32, String, String, String)>(conn) {
        Ok(_) => true,
        Err(Error::NotFound) => false,
        Err(_) => false,
    }
}