// This module handles loading all manifest files and converting files from the
// manifest interchange format to the in memory strucutred format used in later
// operations.
use config::{Suite};
use interchange::{Manifest, ManifestSuite};
use error::{OverlordError, OverlordResult};
use serialize::{Decodable};
use std::str;
use toml;
use std::io::fs::{File};
use std::collections::hashmap::HashSet;
use std::os;

// In addition to the format itself _how_ the manifest is loaded matter for the
// search algorithms this structure keeps track of where the manifest was loaded
// from, etc...

fn load_manifest(path: &Path) -> OverlordResult<Manifest> {
  // Load the contents of the manifest...
  let bytes = match File::open(path).read_to_end() {
    Ok(v) => v,
    Err(e) => {
      return Err(OverlordError::new(format!(
        "Failed to read file: '{}' {}",
         path.display(),
         e.desc
      )));
    }
  };

  let content = match str::from_utf8(bytes.as_slice()) {
    Some(v) => v,
    None => {
      return Err(OverlordError::new("utf8 conversion error...".to_string()))
    }
  };

  let mut parser = toml::Parser::new(content);

  let toml = match parser.parse() {
    Some(v) => v,
    None => {
      return Err(OverlordError::new("Could not parse toml file".to_string()))
    }
  };

  let mut decoder = toml::Decoder::new(toml::Table(toml));

  // XXX: My intent was to terminte and return here but I ended up assigning
  // since I could not figure out how to annotate the decode result as
  // interchange::Manifest without the assignment.
  let manifest: Manifest = match Decodable::decode(&mut decoder) {
    Ok(v) => v,
    // TODO: Expand error messages.
    Err(e) => {
      return Err(OverlordError::new(format!(
        "Error decoding message for '{}' {}",
        path.display(), e
      )))
    }
  };

  Ok(manifest)
}

// Convert the toml format into the in memory config format.
fn convert_manifest_suite<'a>(path: &Path, suite: &ManifestSuite) -> Suite<'a> {
  let root = path.dir_path();
  Suite {
    root: root,
    group: suite.group.clone(),
    paths: suite.paths.iter().map(|path| {
      path.clone()
    }).collect(),
    executable: suite.executable.clone()
  }
}

fn issue_import(
  path: Path, seen_paths: &mut HashSet<Path>
) -> OverlordResult<Vec<Suite>> {

  // Check for circular dependencies.
  if seen_paths.contains(&path) {
    // TODO: Reconsider returning an error on seen paths.
    return Ok(Vec::new());
  }

  // Ensure we don't load the same manifest twice.
  seen_paths.insert(path.clone());

  // Attempt to load the module for the seen path.
  let manifest = try!(load_manifest(&path));
  let mut suites = Vec::new();

  // Not all manifests have suites so handle both cases.
  if manifest.suites.is_some() {
    for suite in manifest.suites.unwrap().iter() {
      suites.push(convert_manifest_suite(&path, suite));
    }
  }

  // The manifest _may_ contain other manifests if so we need to import those as
  // well.
  if manifest.manifests.is_none() {
    Ok(suites)
  } else {
    for sub_manifest in manifest.manifests.unwrap().iter() {
      let sub_manifest_path = Path::new(sub_manifest.as_slice());
      let absolute_manifest_path = path.dir_path().join(sub_manifest_path);
      let sub_suites = try!(issue_import(absolute_manifest_path, seen_paths));
      suites.push_all_move(sub_suites);
    }
    Ok(suites)
  }
}

pub fn import<'a>(path: Path) -> OverlordResult<Vec<Suite<'a>>> {
  let normalized_path = os::make_absolute(&path);
  // Phase 1 is to load _all_ manifests recursively. (without loading the same
  // file twice!)
  let mut seen_paths = HashSet::new();

  // Always insert the root to avoid tricky stuff...
  issue_import(normalized_path, &mut seen_paths)
}

#[cfg(test)]
mod tests {
  use test::{assert_path_eq};
  use config_loader::{import};

  #[test]
  fn load_simple_manifest() {
    let suites = import(Path::new("test/simple/overlord.toml")).unwrap();

    assert_eq!(suites.len(), 1);
    let ref suite = suites[0];

    assert_path_eq(&Path::new("test/simple"), &suite.root);
    assert_eq!(vec!["files/*.txt".to_string()], suite.paths);
    assert_eq!(suite.executable, "cat".to_string());
    assert_eq!(suite.group, "unit".to_string());
  }

  #[test]
  fn nested_manifest() {
    // Note that this main manifest has no suites...
    let suites = import(Path::new("test/multimanifest/overlord.toml")).unwrap();
    assert_eq!(suites.len(), 4);

    // Note that all suites are ordered so it is easy to assert root paths here.
    assert_path_eq(&Path::new("test/multimanifest/1"), &suites[0].root);
    assert_path_eq(&Path::new("test/multimanifest/2"), &suites[1].root);
    assert_path_eq(&Path::new("test/multimanifest/nested"), &suites[2].root);
    assert_path_eq(
      &Path::new("test/multimanifest/nested/another"), &suites[3].root
    );
  }

  #[test]
  fn circular_references() {
    // Note that we don't throw an error but do our best effort to load suites.
    let suites = import(Path::new("test/circular/overlord.toml")).unwrap();
    assert_eq!(suites.len(), 1);
  }
}
