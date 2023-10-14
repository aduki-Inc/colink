use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Selectable)]
// #[table(name = "users")]
#[diesel(table_name = create::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: bool,
  pub bio: String,
  pub dob: NaiveDateTime,
  pub picture: String,
  pub created_at: NaiveDateTime,
}