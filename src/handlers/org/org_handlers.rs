use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
// use diesel::prelude::*;
// use diesel::result::{Error, DatabaseErrorKind};
use chrono::NaiveDate;
use crate::db::connection::establish_connection;
// use crate::db::schema::institutions;
// use crate::db::schema::institutions::dsl::*;
use crate::models::orgs::{NewInstitution, InsertableInstitution };
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::auth_middleware::{JwtMiddleware, Claims};
use crate::middlewares::org::creation_middleware::*;


// Handler for creating new institution
pub async fn create_institution(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, institution_data: web::Json<NewInstitution>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match institution_data.validate() {
      Ok(new_institution) => {

        let established_str = new_institution.established.unwrap();

        let established_date: Option<NaiveDate> = match NaiveDate::parse_from_str(&established_str, "%Y-%m-%d"){
          Ok(created_date) => Some(created_date),
          Err(_) => None,
        };

        let institution = InsertableInstitution {
          short_name: new_institution.short_name,
          in_type: new_institution.in_type,
          name: new_institution.name,
          active: new_institution.active,
          established: established_date
        };

        match institution_exists(&institution.short_name, &institution.name, &mut conn) {
          Ok(true) => {
            return HttpResponse::Conflict().json(
              json!({
                "success": false,
                "message": "Similar Institution already exists!"
              })
            )
          }

          Ok(false) => {
            match institution_created(&user.id, &institution, &mut conn) {
              Ok(created_institution) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "institution": created_institution,
                    "message": format!("Institution: {} created successfully!", &created_institution.name)
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could not create the institution: An error occurred during the process!"
                  })
                )
              }
            }
          }

          Err(_) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not create the Institution: An internal has occurred!"
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