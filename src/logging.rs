//! Contains code for initializing logging in Boardcast.

use std::str::FromStr;
use tracing::level_filters::LevelFilter;

pub fn init_logging(log_level: &str) {
  let level_filter = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::INFO);

  tracing_subscriber::fmt()
    .with_max_level(level_filter)
    .init();
}
