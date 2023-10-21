use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
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


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct LoggedUser {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct LoginData {
  pub user_key: String,
  pub password: String,
}