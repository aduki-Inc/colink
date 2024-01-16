use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
// use crate::db::schema::users::dsl::*;
use diesel::result::Error;
use chrono::{Utc, Duration, NaiveDateTime};
use crate::db::connection::establish_connection;
use crate::db::schema::roles;
use crate::models::system::{ Role, NewRole, InsertableRole, RoleId, RolePrivileges, RoleExpiry };
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::{auth_middleware::{JwtMiddleware, Claims}, role_middleware::* };


// Handler for creating new Role
pub async fn create_role(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, role_data: web::Json<NewRole>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let _user = &claims.user;

    // Collect Role data from the body
    match role_data.validate() {
      Ok(role) => {
        // Check if the Role already exists
        match role_exists(&role.author, &role.section, &mut conn) {
          Ok(true) => {
              return HttpResponse::Conflict().json(
                json!({
                  "success": false,
                  "message": "Same role already exists!"
                })
              );
            }
          Ok(false) => {

            // If expiry days are supplied convert to future date from today
            let expiry_date: Option<NaiveDateTime> = if role.expiry.is_some() {
              let days_to_be_added: i64 = role.expiry.unwrap_or(0);
              let initial_date = Utc::now();

              let future_date = initial_date + Duration::days(days_to_be_added);

              Some(future_date.naive_utc())
            } else {
              None
            };


            // No existing role - creating one
            let new_role = InsertableRole {
              section: role.section,
              base: role.base,
              name: role.name,
              author: role.author,
              privileges: role.privileges,
              expiry: expiry_date,
            };
    
            match diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result::<Role>(&mut conn)
            {
              Ok(role) => return HttpResponse::Ok().json(
                json!({
                  "success": true,
                  "role": role,
                  "message": format!("Role - ({}) - was created successfully", &role.name)
                })
              ),
              Err(err) => {
                // Handle the database error and return an error response
                return	HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": format!("Failed to create the role: {}", err.to_string())
                  })
                )
              }
            }

          },
          Err(_) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Internal server error has ocurred!"
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


// Handler for deleting existing role
pub async fn delete_role(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, role_data: web::Json<RoleId>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let _user = &claims.user;

    // Attempt to delete the role
    match role_deleted(&role_data.id, &mut conn) {
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
	else {
		return HttpResponse::BadRequest().json(
      json!({
        "success": false,
        "message": "Authorization failure!"
      })
    )
	}
}


// Handler for updating expiry date
pub async fn update_expiry(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, role_data: web::Json<RoleExpiry>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;

  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let _user = &claims.user;

    let role_expiry = role_data.into_inner();

    // Check if the section already exists
    match expiry_updated(&role_expiry, &mut conn) {
      Ok(updated_role) => {
        return HttpResponse::Ok().json(
          json!({
            "success": true,
            "role": updated_role,
            "message": format!("Expiry for Role - ({}) - is updated successfully!", &updated_role.name)
          })
        )
      }

      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "No such role was  found!"
          })
        )
      }

      Err(_) => {
        return HttpResponse::InternalServerError().json(
          json!({
            "success": false,
            "message": "Internal server error has occurred while updating role!"
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


// Handler for updating privileges of existing role
pub async fn update_privileges(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, privileges: web::Json<RolePrivileges>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;

  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let _user = &claims.user;

    let role_privileges = privileges.into_inner();

    // Check if the section already exists
    match privileges_updated(&role_privileges, &mut conn) {
      Ok(updated_role) => {
        return HttpResponse::Ok().json(
          json!({
            "success": true,
            "role": updated_role,
            "message": format!("Privileges for Role - ({}) - is updated successfully!", &updated_role.name)
          })
        )
      }

      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "No such role was  found!"
          })
        )
      }

      Err(_) => {
        return HttpResponse::InternalServerError().json(
          json!({
            "success": false,
            "message": "Internal server error has occurred while updating role!"
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