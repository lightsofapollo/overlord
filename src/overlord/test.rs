// Test helpers only exported for internal tests.
use config::Config;
use std::io::fs::{File};
use std::str;

pub static EXAMPLE_PATH: &'static str = "design/overlord.toml";

// Load a manifest configuration file from disk and return a ManifestConfig.
// Remember that all paths are relative to the _root_ of the project.
pub fn config(path: &str) -> Config {
  match Config::parse(&Path::new(path)) {
    Ok(v) => {
      v
    },
    Err(e) => {
      fail!("Failed to convert manifest!: {}", e)
    }
  }
}

pub fn example_config() -> Config {
  config(EXAMPLE_PATH)
}
