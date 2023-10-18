use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::hash;
use crate::db::connection::establish_connection;
use crate::db::schema::users;
use crate::models::users::NewUser;
use serde_json::json;
use crate::middlewares::auth_middleware::{email_exists, username_exists};


// Define an Actix route for user registration with JSON data.
pub async fn register_user(data: web::Json<NewUser>) -> impl Responder {

  let mut conn = establish_connection().await;

  let registration_data = data.into_inner();

  // Check if the email already exists
  if email_exists(&registration_data.email, &mut conn) {
    return HttpResponse::Conflict().json(
        json!({
            "success": false,
            "message": "Email already exists"
        })
    );
  }

  // Check if the username already exists
  if username_exists(&registration_data.username, &mut conn) {
      return HttpResponse::Conflict().json(
          json!({
              "success": false,
              "message": "Username already exists"
          })
      );
  }


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

  match diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&mut conn)
  {
    Ok(_) => HttpResponse::Ok().json(
      json!({
        "success": true,
        "message": "User registered successfully"
      })
    ),
    Err(err) => {
      // Handle the database error and return an error response
      HttpResponse::InternalServerError().json(
        json!({
          "success": false,
          "error": format!("Failed to register user: {}", err.to_string())
          })
      )
    }
  }
}
