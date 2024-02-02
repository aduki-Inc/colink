use crate::db::platform::platform::roles::dsl::*;
use crate::models::system::Section;
use crate::models::{system::{Role, RolePrivileges}, custom_types::RoleType};
use crate::models::orgs::OrgPermission;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use chrono::Utc;

// Check the role for user attempting to create, edit, or delete other roles
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
// pub fn check_member_authority(user_id: &i32, section_id: &i32, permission: &OrgPermission, conn: &mut PgConnection) -> Result<bool, Error> {
//   match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
//     Ok(role) => {
//       // println!("{:?}", &role);
//       match role.privileges.expect("REASON").get(&permission.title) {
//        Some(members) => {
//         match members.as_array().and_then(|arr| arr.iter().find(|&v|v == &permission.name)){
//           Some(_delete_permission) => Ok(true),
//           None => Ok(false)
//         }
//        }
//        None => Ok(false)
//       }
//     },
//     Err(Error::NotFound) => Ok(false),
//     Err(err) => Err(err),
//   }
// }


// Check the role for user attempting to create, edit or delete other roles
pub fn check_org_authority(
  user_id: &i32, section_name: &str, 
  permission: &OrgPermission, 
  conn: &mut PgConnection) -> Result<(bool, Option<Section>), Error> {
  use crate::db::schema::sections::dsl::*;
  match sections
    .filter(identity.eq(section_name))
    .inner_join(roles)
    .filter(author.eq(user_id))
    .select((Section::as_select(), Role::as_select()))
    .load::<(Section, Role)>(conn){
      Ok(section_with_role) => {

        if let Some((section_data, role_data)) = section_with_role.into_iter().next(){
          // println!("{:?}", &section_data);
          // println!("{:?}", &role_data);
          match role_data.privileges.expect("REASON").get(&permission.title) {
            Some(members) => {
              match members.as_array().and_then(|arr| arr.iter().find(|&v|v == &permission.name)){
                Some(_delete_permission) => Ok((true, Some(section_data))),
                None => Ok((false, Some(section_data)))
              }
            }
            None => Ok((false, Some(section_data)))
          }
        }
        else {
          return Ok((false, None))
        }
      },
      Err(Error::NotFound) => Err(Error::NotFound),
      Err(err) => Err(err),
    }
}


pub fn role_exists(user_id: &i32, section_id: &i32, conn: &mut PgConnection) -> Result<bool, Error> {
  match roles.filter(author.eq(user_id).and(section.eq(section_id))).first::<Role>(conn) {
    Ok(_role) => Ok(true),
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


pub fn role_belong_set_expired(role_author: &i32, role_section: &i32, conn: &mut PgConnection) -> Result<Role, Error> {
  let initial_date = Utc::now();

  let expiry_date = Some(initial_date.naive_utc());

  match diesel::update(roles.filter(author.eq(role_author).and(section.eq(role_section))))
  .set(expiry.eq(&expiry_date))
  .get_result::<Role>(conn) {
    Ok(role) => Ok(role),
    Err(err) => Err(err)
  }
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