use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::{ prelude::*, result::Error};
use crate::db::{
	connection::establish_connection,
	platform::platform::sections,
};
use crate::models::platform::{Section, NewSection, SectionIdentity};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::{auth::{JwtMiddleware, Claims}, section::* };

// Handler for creating a new section/org section
pub async fn create_section(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, section_data: web::Json<NewSection>) -> impl Responder {
	//  Get extensions
	let ext = req.extensions();
	let mut conn = establish_connection(&app_data.config.database_url).await;
	
	// Use the 'get' method to retrieve the 'Claims' value from extensions
	let claims = match ext.get::<Claims>() {
		Some(claims) => claims,
		None => {
			return HttpResponse::BadRequest().json(json!({
				"success": false,
				"message": "Authorization failure!"
			}));
		}
	};
	
	// Access 'user' from 'Claims'
	let _user = &claims.user;
	
	// Collect Registration data from the body
	let section = match section_data.validate() {
		Ok(section) => section,
		Err(err) => {
			return HttpResponse::ExpectationFailed().json(json!({
				"success": false,
				"message": err.to_string()
			}));
		}
	};
	
	// Check if the section already exists
	if section_exists(&section.name, &mut conn) {
		return HttpResponse::Conflict().json(json!({
			"success": false,
			"message": "Similar section already exists"
		}));
	}
	
	match diesel::insert_into(sections::table)
		.values(&section)
		.get_result::<Section>(&mut conn)
	{
		Ok(section) => HttpResponse::Ok().json(json!({
			"success": true,
			"section": section,
			"message": "Section added successfully"
		})),
		Err(err) => HttpResponse::InternalServerError().json(json!({
			"success": false,
			"message": format!("Failed to add section: {}", err)
		})),
	}
}


// Handler for deleting existing section
pub async fn delete_section(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, section_data: web::Json<SectionIdentity>) -> impl Responder {
	//  Get extensions
	let ext = req.extensions();
	let mut conn = establish_connection(&app_data.config.database_url).await;
	
	// get claims: if not found return unauthorized
	let claims = match ext.get::<Claims>() {
		Some(claims) => claims,
		None => {
			return HttpResponse::BadRequest().json(json!({
				"success": false,
				"message": "Authorization failure!"
			}));
		}
	};
	
	// Access 'user' from 'Claims'
	let _user = &claims.user;
	
	// validate section data
	let section = match section_data.validate() {
		Ok(section) => section,
		Err(err) => {
			return HttpResponse::ExpectationFailed().json(json!({
				"success": false,
				"message": err.to_string()
			}));
		}
	};
	
	// Check if the section already exists
	if !section_exists(&section.identity, &mut conn) {
		return HttpResponse::NotFound().json(json!({
			"success": false,
			"message": "Section does not exists"
		}));
	}
	
	// delete section
	match section_deleted(&section.id, &section.identity, &mut conn) {
		Ok(true) => {
			HttpResponse::Ok().json(json!({
				"success": true,
				"message": "Section deleted successfully"
			}))
		},
		Ok(false) => {
			HttpResponse::NotFound().json(json!({
				"success": false,
				"message": "Section does not exists"
			}))
		},
		Err(err) => {
			HttpResponse::InternalServerError().json(json!({
				"success": false,
				"message": format!("Failed to delete section: {}", err)
			}))
		},
	}
}

// Handler for updating existing section
pub async fn update_section(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, section_data: web::Json<Section>) -> impl Responder {
	//  Get extensions
	let ext = req.extensions();
	let mut conn = establish_connection(&app_data.config.database_url).await;
	
	// Use the 'get' method to retrieve the 'Claims' value from extensions
	let claims = match ext.get::<Claims>() {
		Some(claims) => claims,
		None => {
			return HttpResponse::BadRequest().json(json!({
				"success": false,
				"message": "Authorization failure!"
			}));
		}
	};
	
	// Access 'user' from 'Claims'
	let _user = &claims.user;
	
	// Collect Section data from the body
	let section = section_data.into_inner();
	
	if section.id <= 0 {
		return HttpResponse::ExpectationFailed().json(
			json!({
				"success": false,
				"message": "Section validation error: zero(0) was encountered for value(id)"
			})
		)
	}
	
	// Check if the section already exists
	if !section_exists(&section.id, &mut conn) {
		return HttpResponse::NotFound().json(json!({
			"success": false,
			"message": "Section does not exists"
		}));
	}
	
	// update section
	match section_updated(&section.id, &section, &mut conn) {
		Ok(updated_section) => {
			HttpResponse::Ok().json(json!({
				"success": true,
				"section": updated_section,
				"message": "Section updated successfully"
			}))
		},
		Err(err) => {
			HttpResponse::InternalServerError().json(json!({
				"success": false,
				"message": format!("Failed to update section: {}", err)
			}))
		},
	}
}