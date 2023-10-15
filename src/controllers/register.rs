use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use std::env;
use bcrypt::{hash, verify};
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};


// Define an Actix route for user registration with JSON data.
async fn register_user(data: web::Json<NewUser>, db: web::Data<DbPool>) -> impl Responder {
  let registration_data = data.into_inner();

  // Hash the user's password securely using bcrypt.
  let hashed_password = match hash(&registration_data.password, 12) {
    Ok(h) => h,
    Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Password hashing failed"})),
  };

  let new_user = NewUser {
    username: registration_data.username,
    email: registration_data.email,
    password: hashed_password,
    name: registration_data.name,
  };

  let conn = db.get().expect("Failed to get a database connection");

  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&conn)
    .expect("Error inserting new user");

  HttpResponse::Ok().json(json!({"message": "User registered successfully"}))
}
