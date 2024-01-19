use crate::db::schema::roles::dsl::*;
use crate::models::{system::{Role, RolePrivileges}, custom_types::RoleType};
use crate::models::orgs::OrgPermission;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;


// Check the role for user attempting to create, edit or delete other roles
pub fn check_authority(user_id: &i32, section_id: &i32, role_type: &RoleType, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
    Ok(role) => {
      match role.base {
        RoleType::Owner => Ok(true),
        RoleType::Admin => {
          match role_type {
            RoleType::Owner => Ok(false),
            RoleType::Admin => Ok(false),
            _=>Ok(true)
          }
        }
        _=> Ok(false)
      }
    },
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}


// Check the role for user attempting to create, edit or delete other roles
pub fn check_member_authority(user_id: &i32, section_id: &i32, permission: &OrgPermission, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
    Ok(role) => {
      match role.privileges.get(permission.title) {
       Some(members) => {
        match members.as_array().and_then(|arr| arr.iter().find(|&v|v == permission.name)){
          Some(delete_permission) => Ok(true),
          None => Ok(false)
        }
       }
       None => Ok(false)
      }
    },
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}


pub fn role_exists(user_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
    Ok(_) => Ok(true),
    Err(Error::NotFound) => Ok(false),
    Err(err) => Err(err),
  }
}

pub fn role_deleted(other_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {

  match diesel::delete(roles.filter(id.eq(other_id))).execute(conn) {
    Ok(1) => Ok(true),
    Ok(0) => Ok(false),
    Err(err) => Err(err),
    Ok(_) => Ok(false)
  }
}


pub fn role_belong_deleted(role_author: &i32, role_section: &i32, conn: &mut PgConnection) -> Result(bool, Error) {
  match diesel::delete(roles.filter(author.eq(role_author).and(section.eq(role_section))))
  .execute(conn) {
    Ok(1) => Ok(true),
    Ok(0) => Ok(false),
    Err(err) => Err(err),
    Ok(_) => Ok(false)
  }s
}

pub fn privileges_updated(new_data: &RolePrivileges, conn: &mut PgConnection) -> Result<Role, Error> {
  match diesel::update(roles.filter(id.eq(new_data.id)))
  .set((
    base.eq(&new_data.base),
    privileges.eq(&new_data.privileges)
  ))
  .get_result(conn) {
    Ok(role) => Ok(role),
    Err(Error::NotFound) => Err(Error::NotFound),
    Err(err) => Err(err)
  }
}


pub fn expiry_updated(role_data: &Role, conn: &mut PgConnection) -> Result<Role, Error> {

  match diesel::update(roles.filter(id.eq(role_data.id)))
  .set(expiry.eq(role_data.expiry))
  .get_result(conn) {
    Ok(role) => Ok(role),
    Err(err) => Err(err)
  }
}