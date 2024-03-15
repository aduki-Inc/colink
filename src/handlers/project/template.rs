use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::project::NewTemplate;
use crate::configs::state::AppState;
// use diesel::result::Error;
use serde_json::json;
use crate::middlewares::{
  auth::auth_middleware::{JwtMiddleware, Claims},
  project::template::template_created
};


// Handler for creating new template
pub async fn create_template(
  req: HttpRequest, _: JwtMiddleware,
  app_data: web::Data<AppState>,
  req_template: web::Json<NewTemplate>) -> impl Responder {

  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let template_data = req_template.into_inner();

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