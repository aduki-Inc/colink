use crate::configs::config::Config;
use std::sync::Mutex;

pub struct AppState {
  pub counter: Mutex<i32>,
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

