use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
// use diesel::prelude::*;
// use diesel::result::{Error, DatabaseErrorKind};
use chrono::NaiveDate;
use crate::db::connection::establish_connection;
use crate::models::orgs::{
  InsertableBelong, BelongIntermediate,
  InsertableOrganization, NewBelong, NewOrganization 
};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::{auth_middleware::{JwtMiddleware, Claims}, role_middleware::{role_exists, check_authority}};
use crate::models::custom_types::{OrgType, RoleType};
use crate::middlewares::org::creation_middleware::*;


// Handler for creating new Organization
pub async fn create_org(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, org_data: web::Json<NewOrganization>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match org_data.validate() {
      Ok(new_org) => {

        let org_name: String = match new_org.base {
          OrgType::Ist => "Institution".to_owned(),
          OrgType::Org => "Organization".to_owned()
        };

        let established_str = new_org.established.unwrap();

        let established_date: Option<NaiveDate> = match NaiveDate::parse_from_str(&established_str, "%Y-%m-%d"){
          Ok(created_date) => Some(created_date),
          Err(_) => None,
        };

        let org = InsertableOrganization {
          short_name: new_org.short_name,
          in_type: new_org.in_type,
          base: new_org.base,
          name: new_org.name,
          active: new_org.active,
          established: established_date
        };

        match org_exists(&org.short_name, &org.name, &mut conn) {
          Ok(true) => {
            return HttpResponse::Conflict().json(
              json!({
                "success": false,
                "message": format!("Similar {} with similar name or short_name(abbr) already exists!", &org_name)
              })
            )
          }

          Ok(false) => {
            match org_created(&user.id, &user.full_name, &org, &mut conn) {
              Ok(created_org) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "org": created_org,
                    "message": format!("{}: {} created successfully!", &org_name, &created_org.name)
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": format!("Could not create the {}: An error occurred during the process!", &org_name)
                  })
                )
              }
            }
          }

          Err(_) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": format!("Could not create the {}: An internal has occurred!", &org_name)
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


// Handler Add new member for an Organization
pub async fn add_org_member(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, org_data: web::Json<NewBelong>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match org_data.validate() {
      Ok(belong_data) => {

        let role_type: RoleType = match belong_data.staff.unwrap() {
          true => RoleType::Staff,
          false => RoleType::Period
        };

        // Check if the user is authorized to perform this action
        match check_authority(&user.id, &belong_data.section, &role_type, &mut conn) {
          Ok(true) => {
            match role_exists(&belong_data.author, &belong_data.section, &mut conn) {
              Ok(true) => {
                return HttpResponse::Conflict().json(
                  json!({
                    "success": false,
                    "message": "Seems like the user you're trying to add already exists!"
                  })
                )
              }
              Ok(false) => {
                let intermediate = BelongIntermediate {
                  user: user.id,
                  section: belong_data.section,
                  date: belong_data.date
                };


                let belong = InsertableBelong {
                  author: belong_data.author,
                  org: belong_data.org,
                  name: belong_data.name,
                  identity: belong_data.identity,
                  title: belong_data.title,
                  staff: belong_data.staff,
                };

                match belongs_created(&intermediate, &belong, &mut conn) {
                  Ok(created_belong) => {
                    return HttpResponse::Ok().json(
                      json!({
                        "success": true,
                        "belong": created_belong,
                        "message": format!("{} was successfully added as a {} ", &created_belong.name, &created_belong.title)
                      })
                    )
                  }
                  Err(_) => {
                    return  HttpResponse::InternalServerError().json(
                      json!({
                        "success": false,
                        "message": "Could not add member: An error occurred during the process!"
                      })
                    )
                  }
                }
    
              }
    
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not add member: An error occurred during the process!"
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
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not add member: An error occurred during the process!"
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