use suite::{Suite};
use config::{Config};
use std::io::fs::{walk_dir};
use error::{OverlordResult};

struct Manifest<'a> {
  suites: Vec<Suite<'a>>
}

impl<'a> Manifest<'a> {

  fn read_manifests
    (path: &Path, suites: &mut Vec<Suite>) -> OverlordResult<()> {

    // Parse the configuration from the path...
    let config = try!(Config::parse(path));

    // Append all suites to the final result.
    for (name, _) in config.manifest.suites.iter() {
      let cur_suite = try!(Suite::from_config(name, &config));
      suites.push(cur_suite);
    }

    // TODO: Detailed error handling we should be able to report missing paths,
    // etc.. without as much detail as possible.
    match config.manifest.manifests {
      None => return Ok(()),
      Some(manifests) => {
        let root = path.dir_path();
        for manifest in manifests.iter() {
          let path = Path::new(manifest.as_slice());
          try!(Manifest::read_manifests(
            &root.join(path), suites
          ));
        }
      }
    };
    Ok(())
  }

  pub fn from_path(path: &Path) -> Manifest {
    // final list of suites from all nested configurations.
    let mut suites: Vec<Suite> = Vec::new();
    Manifest::read_manifests(path, &mut suites);

    Manifest{suites: suites}
  }
}

#[cfg(test)]
mod tests {
  use manifest::{Manifest};
  use test;

  fn multimanifest() -> Manifest {
    Manifest::from_path(&Path::new("test/multimanifest/overlord.toml"))
  }

  #[test]
  fn test_find_manfiests_success() {
    // Find the root configuration.
    multimanifest();
  }
}
