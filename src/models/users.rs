use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: Option<bool>,
  pub bio: Option<String>,
  pub dob: Option<NaiveDateTime>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: Option<bool>,
  pub bio: Option<String>,
  pub dob: Option<NaiveDateTime>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
}

