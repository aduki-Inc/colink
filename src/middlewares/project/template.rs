// use crate::db::project::project::templates::dsl::*;
use crate::db::project::project::templates;
use crate::models::project::{
  Template, InsertableTemplate, NewTemplate
};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

//Middleware for creating new template
pub fn template_created(user_id: &i32, template: NewTemplate, conn: &mut PgConnection) -> Result<Template, Error> {
  let insertable_template = InsertableTemplate {
    author: *user_id,
    name: template.name,
    description: template.description,
    layout: template.layout,
  };
  match diesel::insert_into(templates::table).values(&insertable_template)
  .get_result::<Template>(conn) {
    Ok(template) => Ok(template),
    Err(err) => Err(err)
  }
}