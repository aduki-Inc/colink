use crate::db::schema::roles::dsl::*;
use crate::db::schema::institutions::dsl::*;
use crate::db::schema::approvals::dsl::*;
use crate::db::schema::belongs::dsl::*;
// use crate::db::schema::roles;
use crate::models::{institutions::{Institution, Belong}, system::Approvals};

use crate::models::{system::{Role, RolePrivileges}, custom_types::RoleType};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
// use chrono::{Utc, Duration};



pub fn create_institution(user_id: &i32, &institution: Institution, conn: &mut PgConnection) {
  
}




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