use actix_web::{error, HttpResponse};
use serde_json::json;

pub fn json_cfg(err: actix_web::error::JsonPayloadError) -> actix_web::Error {
	error::InternalError::from_response(
		err,
		HttpResponse::BadRequest().json(json!({
		"success": false,
		"error_type": "deserialization",
		"error": "The payload data could not be validated!",
		}))
	).into()
}