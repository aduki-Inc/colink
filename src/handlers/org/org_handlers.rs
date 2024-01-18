use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
// use diesel::prelude::*;
// use diesel::result::{Error, DatabaseErrorKind};
use chrono::NaiveDate;
use crate::db::connection::establish_connection;
use crate::models::orgs::{InsertableOrganization, NewOrganization };
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::auth_middleware::{JwtMiddleware, Claims};
use crate::models::custom_types::OrgType;
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
            match org_created(&user.id, &org, &mut conn) {
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