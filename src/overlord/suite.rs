use config::{Config};
use error::{OverlordResult, OverlordError};
use glob::{glob, Pattern, MatchOptions};
use std::os;

pub struct Suite<'a> {
  name: String,
  paths: Vec<String>,
}

impl<'a> Suite<'a> {
  pub fn from_config(name: String, config: &Config) -> OverlordResult<Suite> {
    let config_suite = match config.manifest.suites.find(&name) {
      Some(v) => v,
      None => {
        return Err(OverlordError::new(format!(
          "\"{}\" is not a valid suite", name
        )));
      }
    };

    let paths = config_suite.paths.iter().map(|path_ptn| {
      let path = os::make_absolute(&Path::new(path_ptn.as_slice()));
      let abs_path = config.path.join(path);
      abs_path.as_str().unwrap().to_string()
    }).collect::<Vec<String>>();

    Ok(Suite{
      name: name,
      paths: paths
    })
  }

  pub fn contains_path(&self, path: &Path) -> bool {
    let compare_path = os::make_absolute(path);
    let options = MatchOptions {
      case_sensitive: true,

      // This is the important one we don't want to match differently then we
      // glob so this ensures we only match in the same directory. Without this
      // being set to true nested directories will match but are not returned
      // when we use glob.
      require_literal_separator: true,
      require_literal_leading_dot: false
    };
    // TODO: This is potentially a hot path and this is the least optimal thing
    // we could do.
    self.paths.iter().any(|path_str| {
      let pattern = Pattern::new(path_str.as_slice());
      println!("{} == {}", compare_path.display(), path_str);
      pattern.matches_path_with(&compare_path, options)
    })
  }
}

#[cfg(test)]
mod tests {
  use config::{Config};
  use suite::{Suite};
  use std::os;
  use glob::glob;
  use test;

  fn glob_suite() -> (Config, Suite) {
    let config = test::config("test/globs/overlord.toml");
    let suite = match Suite::from_config("globs".to_string(), &config) {
      Ok(v) => v,
      Err(e) => fail!("Failed to find suite {} {}", "globs", e)
    };

    return (config, suite);
  }

  // Sorts and converts paths to strings...
  fn vec_path_to_string(paths: Vec<Path>) -> Vec<String> {
    let mut sortable = paths.clone();
    sortable.sort_by(|a, b| a.cmp(b));
    sortable.iter().
      map(|x| {
        os::make_absolute(x).as_str().unwrap().to_string()
      }).
      collect()
  }

  #[test]
  fn test_contains_path() {
    let (config, suite) = glob_suite();
    let matches_rel_path = Path::new("folder/bar.txt");
    let no_match = Path::new("folder/nested/forgotten.txt");

    // Relative paths should also be matched.
    assert_eq!(suite.contains_path(&matches_rel_path), true);

    // Absolute paths should be matched.
    assert_eq!(
      suite.contains_path(&os::make_absolute(&matches_rel_path)), true);

    assert_eq!(suite.contains_path(&no_match), false);
  }
}
