use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::orgs::{
  EditBelong, BelongIdentity, OrgPermission, BelongStaff
};
use crate::configs::state::AppState;
use diesel::result::Error;
use serde_json::json;
use crate::middlewares::auth::{
  auth::{JwtMiddleware, Claims},
  role::{ check_org_authority, role_belong_set_expired }
};
use crate::middlewares::org::member::*;

// Handler for editing member info
pub async fn edit_user_info(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>,
  path: web::Path<String>,
  edit_data: web::Json<EditBelong>
) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match edit_data.validate() {
      Ok(belong_data) => {

        let req_permission = OrgPermission {
          title: "members".to_owned(),
          name: "update".to_owned()
        };
        // Check if the user is authorized to perform this action
        match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
          Ok((true, Some(_section))) => {
            match belong_edited(&belong_data, &mut conn) {
              Ok(belong) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "belong": belong,
                    "message": "User Details was changed successfully!"
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
          Err(_) => {
            return  HttpResponse::Unauthorized().json(
              json!({
                "success": false,
                "message": "Could not verify your authority: An error occurred during the process!"
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


// Handler for editing member staff status
pub async fn edit_staff_status(
  req: HttpRequest, _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  status_data: web::Json<BelongStaff>) -> impl Responder {


  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let belong_data = status_data.into_inner();
    let req_permission = OrgPermission {
      title: "staff".to_owned(),
      name: "update".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(_section))) => {
        match belong_staff_edited(&belong_data.id, &belong_data.staff, &mut conn) {
          Ok(belong) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "belong": belong,
                "message": "Member Staff status was changed successfully!"
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
                "message": "Could not edit member staff status: An error occurred during the process!"
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


// Handler for deactivating member 
pub async fn disable_user(
  req: HttpRequest, _: JwtMiddleware, 
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  status_data: web::Json<BelongIdentity>) -> impl Responder {

  //Extract from path
  let org  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let belong_data = status_data.into_inner();
    let req_permission = OrgPermission {
      title: "members".to_owned(),
      name: "delete".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(section))) => {

        match is_member_active(&belong_data.id, &mut conn) {
          Ok(true) => {
            match role_belong_set_expired(&belong_data.author, &section.id, &mut conn) {
              Ok(role) => {
                match member_disabled(&belong_data.id, &mut conn) {
                  Ok(belong) => {
                    return HttpResponse::Ok().json(
                      json!({
                        "success": true,
                        "role": role,
                        "belong": belong,
                        "message": "User is no longer active member in this organization!"
                      })
                    )
                  }
                  Err(Error::NotFound) => {
                    return HttpResponse::NotFound().json(
                      json!({
                        "success": false,
                        "message": "User is not yet a member of this organization!"
                      })
                    )
                  }
                  Err(_) => {
                    return  HttpResponse::InternalServerError().json(
                      json!({
                        "success": false,
                        "message": "Could not remove user: An error occurred during the process!"
                      })
                    )
                  }
                }
    
              }
              Err(Error::NotFound) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "User does not have a role in this organization!"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not remove user: An error occurred during the process!"
                  })
                )
              }
            }
          }
          Ok(false) => {
            return HttpResponse::Conflict().json(
              json!({
                "success": false,
                "message": "User is already inactive in this organization!"
              })
            )
          }

          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not remove user: An error occurred during the process!"
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


// Handler for re enabling disabled  member 
pub async fn enable_user(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>, 
  path: web::Path<String>,
  status_data: web::Json<BelongIdentity>) -> impl Responder {


  //Extract from path
  let org  = path.into_inner();


  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let belong_data = status_data.into_inner();
    let req_permission = OrgPermission {
      title: "members".to_owned(),
      name: "delete".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_org_authority(&user.id, &org, &req_permission, &mut conn) {
      Ok((true, Some(section))) => {

        match is_member_active(&belong_data.id, &mut conn) {
          Ok(true) => {
            return HttpResponse::Conflict().json(
              json!({
                "success": false,
                "message": "User is already active in this organization!"
              })
            )
          }
          Ok(false) => {
            match role_belong_set_expired(&belong_data.author, &section.id, &mut conn) {
              Ok(role) => {
                match member_enabled(&belong_data.id,  &mut conn) {
                  Ok(belong) => {
                    return HttpResponse::Ok().json(
                      json!({
                        "success": true,
                        "role": role,
                        "belong": belong,
                        "message": "User is now an active member in this organization!"
                      })
                    )
                  }
                  Err(Error::NotFound) => {
                    return HttpResponse::NotFound().json(
                      json!({
                        "success": false,
                        "message": "User is not yet a member of this organization!"
                      })
                    )
                  }
                  Err(_) => {
                    return  HttpResponse::InternalServerError().json(
                      json!({
                        "success": false,
                        "message": "Could not remove user: An error occurred during the process!"
                      })
                    )
                  }
                }
    
              }
              Err(Error::NotFound) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "User does not have a role in this organization!"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not remove user: An error occurred during the process!"
                  })
                )
              }
            }
          }

          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not remove user: An error occurred during the process!"
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