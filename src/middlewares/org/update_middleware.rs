use crate::db::schema::orgs::dsl::*;
use crate::models::orgs::Organization;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// Updating the Organization/Institution Logo
pub fn org_logo_updated(file_url: &str, org_short_name: &str, conn: &mut PgConnection) -> Result<Organization, Error> {
  match diesel::update(orgs.filter(short_name.eq(org_short_name)))
  .set(logo.eq(file_url))
  .get_result::<Organization>(conn) {
    Ok(org) => Ok(org),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}