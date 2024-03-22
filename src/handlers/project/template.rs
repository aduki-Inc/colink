use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::project::{NewTemplate, EditTemplate};
use crate::configs::state::AppState;
use diesel::result::Error;
use serde_json::json;
use crate::middlewares::{
  auth::auth::{JwtMiddleware, Claims},
  project::template::{template_created, template_edited}
};


// Handler for creating new template
pub async fn create_template(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>,
  payload: web::Json<NewTemplate>
) -> impl Responder {

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let template_data = payload.into_inner();

    match template_created(&user.id, template_data, &mut conn) {
      Ok(template) => {
        return HttpResponse::Ok().json(
          json!({
            "success": true,
            "template": template,
            "message": format!("Template - {} - was changed successfully!", &template.name)
          })
        )
      }
      Err(_) => {
        return  HttpResponse::InternalServerError().json(
          json!({
            "success": false,
            "message": "Could add the template: An error occurred during the process!"
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


// Handler for  updating existing template
pub async fn update_template(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>,
  path: web::Path<i32>,
  payload: web::Json<EditTemplate>
) -> impl Responder {

  //Extract from path
  let template_id  = path.into_inner();

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let template_data = payload.into_inner();

    match template_edited(&template_id, &user.id, &template_data, &mut conn) {
      Ok(template) => {
        return HttpResponse::Ok().json(
          json!({
            "success": true,
            "template": template,
            "message": format!("Template - {} - was successfully updated ",  &template.name)
          })
        )
      }
      Err(Error::NotFound) => {
        return HttpResponse::NotFound().json(
          json!({
            "success": false,
            "message": "The template you're trying to update does not exists!"
          })
        )
      }
      Err(_) => {
        return  HttpResponse::InternalServerError().json(
          json!({
            "success": false,
            "message": "Could not update the template: An error occurred during the process!"
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