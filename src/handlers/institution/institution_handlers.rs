use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
use diesel::result::{Error, DatabaseErrorKind};
use chrono::{Utc, Duration, NaiveDateTime};
use crate::db::connection::establish_connection;
use crate::db::schema::roles;
use crate::db::schema::roles::dsl::*;
use crate::models::institutions::{ Institution, NewInstitution };
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::{auth_middleware::{JwtMiddleware, Claims}, role_middleware::* };


// Handler for creating new institution
pub async fn create_institution(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, req_data: web::Json<NewInstitution>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match role_data.validate() {
      Ok(role) => {

        match check_authority(&user.id, &role.section, &role.base, &mut conn) {
          Ok(true) => {

            // Attempt to delete the role
            match role_deleted(&role.id, &mut conn) {
              Ok(true) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "message": "Role is deleted successfully!"
                  })
                )
              }

              Ok(false) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "Role does not exists!"
                  })
                )
              }

              Err(_) => {
                return HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Internal server error has occurred!"
                  })
                )
              }
            }
          }

          Ok(false) => {
            return HttpResponse::Forbidden().json(
              json!({
                "success": false,
                "message": "You're not authorized to create the role!"
              })
            )
          }

          Err(_) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Failed to create the role: Internal Error Occurred!"
              })
            )
          }
        }
      },
      Err(err) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": err.to_string()
          })
        )
      }
    }
	}
	else {
		return HttpResponse::BadRequest().json(
      json!({
        "success": false,
        "message": "Authorization failure!"
      })
    )
	}
}