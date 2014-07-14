use config::{Config};
use error::{OverlordResult, OverlordError};
use glob::{glob};

pub struct Suite<'a> {
  name: String,
  paths: Vec<Path>
}

fn expand_paths(root: &Path, globs: &Vec<String>) -> Vec<Path> {
  globs.iter().flat_map(|glob_pattern| {
    let glob_path = Path::new(glob_pattern.as_slice());
    // Use path to join our paths + strings correclty.
    let abs_path = root.join(glob_path);
    let abs_path_str = match abs_path.as_str() {
      Some(v) => v,
      None => fail!("Expected failure when path: {}", abs_path.display())
    };
    glob(abs_path_str)
  }).collect::<Vec<Path>>()
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

    let paths = expand_paths(&config.path.dir_path(), &config_suite.paths);

    Ok(Suite{
      name: name,
      paths: paths,
    })
  }
}

#[cfg(test)]
mod tests {
  use suite::{Suite};
  use std::os;
  use glob::glob;
  use test;

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
  fn test_suite_with_file_globs() {
    let config = test::config("test/globs/overlord.toml");
    let suite = match Suite::from_config("globs".to_string(), &config) {
      Ok(v) => v,
      Err(e) => fail!("Failed to find suite {} {}", "globs", e)
    };

    let root = config.path.dir_path();

    let given = suite.paths;
    let expected = vec!(
      root.join(Path::new("folder/bar.txt")),
      root.join(Path::new("folder/woot.txt"))
    );

    assert_eq!(expected.len(), 2)
    assert_eq!(given.len(), 2)
    assert_eq!(vec_path_to_string(expected), vec_path_to_string(given));
  }
}
