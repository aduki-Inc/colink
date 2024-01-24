use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::orgs::OrgPermission;

use actix_multipart::form::MultipartForm;
use crate::configs::state::AppState;
use diesel::result::Error;
use serde_json::json;
use crate::middlewares::auth::{
  auth_middleware::{JwtMiddleware, Claims},
  role_middleware::{ check_member_authority, role_belong_set_expired }
};
use crate::middlewares::org::update_middleware::*;
use crate::utils::file_util::{ UploadError, upload_file, UploadForm };

// Handler for updating organization logo
pub async fn update_logo(
  req: HttpRequest, 
  _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  payload: MultipartForm<UploadForm>) -> impl Responder {

  //Extract from path
  let org_short_name  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let req_permission = OrgPermission {
      title: "staff".to_owned(),
      name: "update".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_member_authority(&user.id, &belong_data.section, &req_permission, &mut conn) {
      Ok(true) => {
        match upload_file(payload, &org_short_name, "static/orgs/logos").await {
          Ok(file_url) => {
            match org_logo_updated(&file_url, &org_short_name, &mut conn) {
              Ok(org) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "org": org,
                    "message": "Organization logo was uploaded successfully!"
                  })
                )
              }
              Err(Error::NotFound) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "Member is no longer active in this organization"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could change the user information: An error occurred during the process!"
                  })
                )
              }
            }
          }
          Err(err) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": err.to_string()
              })
            )
          }
        }
        
      }

      Ok(false) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(_) => {
        return  HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "Could not verify your authority: An error occurred during the process!"
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
