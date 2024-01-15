use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
// use crate::db::schema::users::dsl::*;
// use diesel::result::Error;
use crate::db::connection::establish_connection;
use crate::db::schema::{users, roles, sections};
use crate::models::{system::{Role, NewRole, InsertableRole, RoleId}, users::User};
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
        if role_exists(&role.name, &role.section, &role.author, &mut conn) {
          return HttpResponse::Conflict().json(
            json!({
              "success": false,
              "message": "Same role already exists!"
            })
          );
        }

        let new_role = InsertableRole {
          section: &role.section,
          type_: &role.type_,
          name: &role.name,
          author: &role.author,
          privileges: &role.privileges,
          expiry: None,
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
            "message": format!("Role - {} - is deleted successfully!", &role_data.name)
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


// Handler for updating existing role
pub async fn update_section(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, section_data: web::Json<Section>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;

  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let _user = &claims.user;

    let section = section_data.into_inner();

    // Check if the section already exists
    match section_updated(&section.id, &section, &mut conn) {
      Ok(updated_section) => {
        return HttpResponse::Ok().json(
          json!({
            "success": true,
            "section": updated_section,
            "message": format!("Section: {} is updated successfully!", &section.name)
          })
        )
      }

      Err(_) => {
        return HttpResponse::InternalServerError().json(
          json!({
            "success": false,
            "message": format!("Internal server error has occurred while updating section: {}!", &section.name)
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