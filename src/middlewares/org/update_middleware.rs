use crate::db::schema::orgs::dsl::*;
use crate::models::orgs::{Organization, OrganizationInfo, OrganizationContact};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// Updating the Organization/Institution Logo
pub async fn org_logo_updated(file_url: &str, org_short_name: &str, conn: &mut PgConnection) -> Result<Organization, Error> {
  match diesel::update(orgs.filter(short_name.eq(org_short_name)))
  .set(logo.eq(file_url))
  .get_result::<Organization>(conn) {
    Ok(org) => Ok(org),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}


// Updating the Organization/Institution Background Image
pub async fn org_background_updated(file_url: &str, org_short_name: &str, conn: &mut PgConnection) -> Result<Organization, Error> {
  match diesel::update(orgs.filter(short_name.eq(org_short_name)))
  .set(picture.eq(file_url))
  .get_result::<Organization>(conn) {
    Ok(org) => Ok(org),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}

// Updating the Organization/Institution Background Image
pub async fn org_info_updated(org_info: &OrganizationInfo, org_short_name: &str, conn: &mut PgConnection) -> Result<Organization, Error> {
  let established_str = org_info.established.unwrap();

  let established_date: Option<NaiveDate> = match NaiveDate::parse_from_str(&established_str, "%Y-%m-%d"){
    Ok(created_date) => Some(created_date),
    Err(_) => None,
  };

  match diesel::update(orgs.filter(short_name.eq(org_short_name)))
  .set((
    name.eq(&org_info.name), 
    location.eq(&org_info.location), 
    about.eq(&org_info.about), 
    established.eq(&established_date)
  ))
  .get_result::<Organization>(conn) {
    Ok(org) => Ok(org),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}

// Updating the Organization/Institution Background Image
pub async fn org_contact_updated(org_contact: &OrganizationContact, org_short_name: &str, conn: &mut PgConnection) -> Result<Organization, Error> {
  match diesel::update(orgs.filter(short_name.eq(org_short_name)))
  .set(contact.eq(&org_contact.contact))
  .get_result::<Organization>(conn) {
    Ok(org) => Ok(org),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}