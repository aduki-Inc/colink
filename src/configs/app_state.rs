use crate::configs::config::Config;

pub struct AppState {
  pub config: Config,
}

impl AppState {
  pub fn new(config: Config) -> Self {
    AppState { config }
  }
}

