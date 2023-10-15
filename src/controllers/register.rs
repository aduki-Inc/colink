use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::hash;
use crate::db::connection::establish_connection;
use crate::db::schema::users;
use crate::models::users::NewUser;
use serde_json::json;
// use serde::{Deserialize, Serialize};


// Define an Actix route for user registration with JSON data.
pub async fn register_user(data: web::Json<NewUser>) -> impl Responder {

  let mut conn = establish_connection().await;

  let registration_data = data.into_inner();

  // Hash the user's password securely using bcrypt.
  let hashed_password = match hash(&registration_data.password, 12) {
    Ok(h) => h,
    Err(_) => return HttpResponse::InternalServerError().json(
      json!({
        "success": false,
        "error": "Password hashing failed"
      })
    ),
  };

  let new_user = NewUser {
    username: registration_data.username,
    email: registration_data.email,
    password: hashed_password,
    name: registration_data.name,
    active: None,
    bio: None,
    dob: None,
    picture: None,
    created_at: None
  };

  diesel::insert_into(users::table)
    .values(&new_user.clone())
    .execute(&mut conn)
    .expect("Error inserting new user");

    HttpResponse::Ok().json(
      json!({
        "success": true,
        "message": "User registered successfully"
      })
    ) 
}
