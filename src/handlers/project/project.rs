use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::project::NewProject;
use crate::configs::state::AppState;
// use diesel::result::Error;
use serde_json::json;
use crate::middlewares::{
  auth::auth_middleware::{JwtMiddleware, Claims},
  project::project::project_created
};


// Handler for creating new template
pub async fn create_project(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>,
  payload: web::Json<NewProject>
) -> impl Responder {

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let project_data = payload.into_inner();

    match project_data.validate() {
      Ok(new_project) => {
        match project_created(&user.id, new_project, &mut conn).await {
          Ok(created_project) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "project": created_project,
                "message": format!("Project - {} - was added successfully!", &created_project.name)
              })
            )
          }
          Err(err) => {
            return  HttpResponse::NotFound().json(
              json!({
                "success": false,
                "message": err.to_string()
              })
            )
          }
        }
      },
      Err(err) => {
        // Directly return the HttpResponse
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": err.to_string()
          })
        )
      },
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
