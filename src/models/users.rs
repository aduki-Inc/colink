use diesel::prelude::*;
use chrono::NaiveDateTime;


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
  pub bio: Option<String>,
  pub dob: NaiveDateTime,
  pub picture: Option<String>,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: bool,
  pub bio: Option<String>,
  pub dob: NaiveDateTime,
  pub picture: Option<String>,
  pub created_at: NaiveDateTime,
}

