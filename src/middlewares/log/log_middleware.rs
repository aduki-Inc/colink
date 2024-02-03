// use crate::db::platform::platform::logs::dsl::*;
use crate::db::platform::platform::logs;
use crate::models::system::InsertableLog;
use diesel::prelude::*;
// use diesel::result::Error;
use diesel::pg::PgConnection;

// Creating logs 
pub async fn create_log(log_data: &InsertableLog, conn: &mut PgConnection) {
  let _ = diesel::insert_into(logs::table).values(log_data).execute(conn);
}