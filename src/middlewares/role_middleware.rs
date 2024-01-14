use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result, dev::Payload, FromRequest, HttpResponse, HttpMessage, ResponseError};
use std::future::{ready, Ready};
use crate::db::schema::sections::dsl::*;
use crate::models::system::Section;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;


pub fn section_exists(other_name: &str, conn: &mut PgConnection) -> bool {
  match sections.filter(name.eq(other_name)).first::<User>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}
