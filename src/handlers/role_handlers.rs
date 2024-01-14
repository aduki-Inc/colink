use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use diesel::result::Error;
use crate::db::connection::establish_connection;
use crate::db::schema::{users, roles, section};
use crate::models::{system::{Colink, Section, Role},users::{User, LoggedUser, NewUser, LoginData, Username}};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth_middleware::{JwtMiddleware, Claims};


// Define handler for user registration with JSON data.
pub async fn create_section(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, section_data: web::Json<Section>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;

	// Use the 'get' method to retrieve the 'Claims' value from extensions
  match ext.get::<Claims>() {
    Ok(claims) => {
      let user_info = &claims.user;

      // Collect Registration data from the body
      match section_data.validate() {
        Ok(section) => {
          // Check if the email already exists
          let new_section = Section {
            id: section.id,
            name: section.name,
            target_id: section.target_id,
            target_name: section.target_name,
            created_at: None,
            updated_at: None
          };

          match diesel::insert_into(section::table)
          .values(&new_section)
          .execute(&mut conn)
          {
            Ok(_) => return HttpResponse::Ok().json(
              json!({
                "success": true,
                "message": "Section added successfully"
              })
            ),
            Err(err) => {
              // Handle the database error and return an error response
              return	HttpResponse::InternalServerError().json(
                json!({
                  "success": false,
                  "error": format!("Failed to add section: {}", err.to_string())
                })
              )
            }
          }
        }
        Err(err) => {
          // Directly return the HttpResponse
          return HttpResponse::BadRequest().json(
            json!({
              "success": false,
              "error": err.to_string()
            })
          )
        }
      }

    }
    Err(_) => {
      return HttpResponse::BadRequest().json(
        json!({
          "success": false,
          "error": "Authorization failed!"
        })
      )
    }
  }
}