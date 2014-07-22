extern crate toml;
use std::collections::HashMap;
use serialize::{Decodable, Encodable};
use error::{OverlordResult, OverlordError};
use std::io::fs::{File};
use std::str;

pub struct Config<'a> {
  // The manifest directly parsed from toml file.
  pub manifest: ManifestConfig<'a>,

  // Location of the configuration file.
  pub path: Path
}

// Struct describing the state of the config suite prior to transforming it.
#[deriving(Decodable, Encodable, Show, Clone)]
pub struct SuiteConfig<'a> {
  // Absolute paths to test files.
  pub paths: Vec<String>,

  // Executable used to run the paths.
  pub executable: String
}

// Struct describing the config.
#[deriving(Decodable, Encodable, Show)]
pub struct ManifestConfig<'a> {
  pub manifests: Option<Vec<String>>,

  // Suites for thie manifest.
  pub suites: HashMap<String, SuiteConfig<'a>>,

  // Optional descritpitons for this manifest.
  pub descriptions: Option<HashMap<String, String>>
}

impl<'a> SuiteConfig<'a> {
}

impl<'a> ManifestConfig<'a> {

  // Convert a toml string to a manifest config with specialized error handling.
  pub fn from_toml_string(contents: &str) -> OverlordResult<ManifestConfig> {
    let mut parser = toml::Parser::new(contents);

    let toml = match parser.parse() {
      Some(v) => v,
      None => {
        // TODO: Expand error messages.
        return Err(OverlordError::new("Toml parse error".to_string()));
      }
    };

    let mut decoder = toml::Decoder::new(toml::Table(toml));
    let manifest: ManifestConfig = match Decodable::decode(&mut decoder) {
      Ok(v) => v,
      // TODO: Expand error messages.
      Err(e) => {
        return Err(
          OverlordError::new(format!("Error decoding message: {}", e))
        )
      }
    };

    return Ok(manifest);
  }
}

impl<'a> Config<'a> {
  pub fn parse(path: &Path) -> OverlordResult<Config> {
    let content = File::open(path).read_to_end().unwrap();
    let fixture = str::from_utf8(content.as_slice()).unwrap();
    return Config::from_toml_string(path, fixture);
  }

  pub fn from_toml_string(path: &Path, toml: &str) -> OverlordResult<Config> {
    let manifest_config = match ManifestConfig::from_toml_string(toml) {
      Err(e) => return Err(e),
      Ok(v) => v
    };

    Ok(Config {path: path.clone(), manifest: manifest_config})
  }
}


#[cfg(test)]
mod test {
  use test::{example_config, EXAMPLE_PATH};

  #[test]
  fn test_manifest_from_string() {
    let subject = example_config();
    assert!(Path::new(EXAMPLE_PATH) == subject.path);
  }
}
