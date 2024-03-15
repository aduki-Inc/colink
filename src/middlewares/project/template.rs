use crate::db::project::project::templates::dsl::*;
use crate::db::project::project::templates;
use crate::models::project::{
  Template, InsertableTemplate, NewTemplate, EditTemplate
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

// Updating template
pub fn template_edited(template_id: &i32, user_id: &i32, template_data: &EditTemplate, conn: &mut PgConnection) -> Result<Template, Error> {
  match diesel::update(templates.filter(id.eq(template_id).and(author.eq(user_id))))
  .set((
    name.eq(&template_data.name),
    description.eq(&template_data.description),
    layout.eq(&template_data.layout)
  ))
  .get_result::<Template>(conn) {
    Ok(template) => Ok(template),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}