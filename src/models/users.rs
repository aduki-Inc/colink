use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: bool,
  pub bio: String,
  pub dob: NaiveDate,
  pub picture: String,
  pub created_at: NaiveDateTime,
}