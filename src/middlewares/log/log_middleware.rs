// use crate::db::platform::platform::logs::dsl::*;
use crate::db::platform::platform::logs;
use crate::models::custom_types::{LogType, ActionType};
use crate::models::system::InsertableLog;
use diesel::prelude::*;
// use diesel::result::Error;
use diesel::pg::PgConnection;

// Creating logs
pub async fn create_log(log_data: &InsertableLog, conn: &mut PgConnection) {
  let _ = diesel::insert_into(logs::table).values(log_data).execute(conn);
}

// pub async fn new_database_error(author: i32, action: ActionType, err: String) -> InsertableLog {
//   return InsertableLog {
//     audit: 	LogType::Error,
//     author,
//     target: 0,
//     name: "database".to_owned(),
//     action,
//     verb: format!("Database Error: {}", &err)
//   };
// }


// Function to create new log: Section Log
pub async fn new_section_log(
  author: i32,
  target: i32,
  action: ActionType,
  verb: String
) -> InsertableLog {
  return InsertableLog {
    audit: LogType::Action,
    author,
    target,
    name: "section".to_owned(),
    action,
    verb,
  };
}
