use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::orgs::{OrgPermission, OrganizationInfo, OrganizationContact };
// use std::path::PathBuf;
use actix_multipart::form::MultipartForm;
use crate::configs::state::AppState;
use diesel::result::Error;
use serde_json::json;
use crate::middlewares::{
  auth::{
    auth_middleware::{JwtMiddleware, Claims},
    role_middleware::check_org_authority
  },
  org::update_middleware::{
    org_logo_updated,
    org_background_updated,
    org_info_updated,
    org_contact_updated
  }
};
use crate::utilities::file_utility::{ upload_file, UploadForm };

// Handler for updating organization logo
pub async fn update_logo(
  req: HttpRequest,
  _: JwtMiddleware,
  app_data: web::Data<AppState>,
  path: web::Path<String>,
  payload: MultipartForm<UploadForm>) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let req_permission = OrgPermission {
      title: "info".to_owned(),
      name: "update".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(_section))) => {
        match upload_file(payload, &org, &app_data.static_dir, "orgs/logos").await {
          Ok(file_url) => {
            match org_logo_updated(&file_url, &org, &mut conn).await {
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
                    "message": "The organization was not found!"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not update the logo: An error occurred during the process!"
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

      Ok((true, None)) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": "The section you are trying to update was not found!"
          })
        )
      }

      Ok((false, Some(_))) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Ok((false, None)) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "Could not verify your authority, or the organization you're trying to update does not exists!"
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



// Handler for updating organization background Image
pub async fn update_background(
  req: HttpRequest, 
  _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  payload: MultipartForm<UploadForm>) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let req_permission = OrgPermission {
      title: "info".to_owned(),
      name: "update".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(_section))) => {
        match upload_file(payload, &org, &app_data.static_dir, "orgs/backgrounds").await {
          Ok(file_url) => {
            match org_background_updated(&file_url, &org, &mut conn).await {
              Ok(org) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "org": org,
                    "message": "Organization background image was uploaded successfully!"
                  })
                )
              }
              Err(Error::NotFound) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "The organization was not found!"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not update the background image: An error occurred during the process!"
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

      Ok((true, None)) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": "The section you are trying to update was not found!"
          })
        )
      }

      Ok((false, Some(_))) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Ok((false, None)) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "Could not verify your authority, or the organization you're trying to update does not exists!"
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



// Handler for updating organization logo
pub async fn update_info(
  req: HttpRequest, 
  _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  org_info: web::Json<OrganizationInfo>) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let req_permission = OrgPermission {
      title: "info".to_owned(),
      name: "update".to_owned()
    };

    let org_data = org_info.into_inner();

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(_section))) => {
        
        match org_info_updated(&org_data, &org, &mut conn).await {
          Ok(org) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "org": org,
                "message": "The organization info was changed successfully!".to_string()
              })
            )
          },
          Err(Error::NotFound) => {
            return HttpResponse::NotFound().json(
              json!({
                "success": false,
                "message": "The organization you're trying to update was not found!"
              })
            )
          }
          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not update the information: An error occurred during the process!"
              })
            )
          }
        }

      }

      Ok((true, None)) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": "The section you are trying to update was not found!"
          })
        )
      }

      Ok((false, Some(_))) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Ok((false, None)) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "Could not verify your authority, or the organization you're trying to update does not exists!"
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



// Handler for updating organization logo
pub async fn update_contact(
  req: HttpRequest, 
  _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  org_info: web::Json<OrganizationContact>) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let req_permission = OrgPermission {
      title: "info".to_owned(),
      name: "update".to_owned()
    };

    let org_data = org_info.into_inner();

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(_section))) => {
        
        match org_contact_updated(&org_data, &org, &mut conn).await {
          Ok(org) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "org": org,
                "message": "The organization info was changed successfully!".to_string()
              })
            )
          },
          Err(Error::NotFound) => {
            return HttpResponse::NotFound().json(
              json!({
                "success": false,
                "message": "The organization you're trying to update was not found!"
              })
            )
          }
          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not update the contact information: An error occurred during the process!"
              })
            )
          }
        }

      }

      Ok((true, None)) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": "The section you are trying to update was not found!"
          })
        )
      }

      Ok((false, Some(_))) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Ok((false, None)) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "Could not verify your authority, or the organization you're trying to update does not exists!"
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