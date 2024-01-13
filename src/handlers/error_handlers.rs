//use actix_web::{error::ErrorHandler, HttpResponse, http::StatusCode, Error, web::Json, HttpRequest};
//use serde_json::json;
//
//pub struct MyErrorHandler;
//
//impl ErrorHandler for MyErrorHandler {
//	fn handle(&self, error: Error, _req: &HttpRequest) -> Error {
//		let response = create_error_response(error);
//		HttpResponse::from_error(error).json(response)
//	}
//}
//
//pub fn create_error_response(error: Error) -> Json<serde_json::Value> {
//	let error_message = match error {
//		// Handle specific errors with tailored messages
//		// ...
//		_ => format!("An unexpected error occurred: {}", error),
//	};
//	json!({ "success": false, "error": error_message }) // Use consistent format
//}
