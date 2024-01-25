use crate::configs::config::Config;
use std::sync::Mutex;
// use std::path::PathBuf;

pub struct AppState {
  pub counter: Mutex<i32>,
  pub static_dir: String,
  pub config: Config,
}

// impl AppState {
//   pub fn init() ->  AppState {
//     let config = Config::init();
//     AppState { 
//       config 
//     }
//   }
// }

