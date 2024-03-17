use actix_web::{error, HttpResponse, Responder};
use serde_json::json;

pub fn json_cfg(err: actix_web::error::JsonPayloadError) -> actix_web::Error {
	error::InternalError::from_response(
		err,
		HttpResponse::BadRequest().json(json!({
			"success": false,
			"error_type": "deserialization",
			"message": "The payload data could not be validated!",
		}))
	).into()
}


// Handler for creating new template
pub async fn url_not_found() -> impl Responder {
	return HttpResponse::MethodNotAllowed().json(
  	json!({
      "success": false,
      "message": "The resource you're trying to access exists!"
    })
  )
}
