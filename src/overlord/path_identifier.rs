// Paths to suites are matched here...
use config::{Suite};

pub fn identify<'a>(
  path: &Path, suites: &'a Vec<Suite>
) -> Option<&'a Suite<'a>> {

  // Deepest meaning the mosted nested in the file system (but with the correct
  // ancestry).
  let mut deepest: Option<&Suite> = None;

  for suite in suites.iter() {
    let root_path = suite.root.get();

    // Rule out any suites which could not possibly match.
    if !root_path.is_ancestor_of(path) {
      continue
    }

    // If this was the first iteration continue to the next for comparision.
    if deepest.is_none() {
      deepest = Some(suite);
      continue
    }

    // If the root path is greater then it is now the new deepest entry.
    if root_path > deepest.unwrap().root.get() {
      deepest = Some(suite);
    }
  };

  // No suite was even a close match based on roots.
  if deepest.is_none() {
    return None;
  }

  // We have the closest suite by root check if any of the paths match.
  let suite = deepest.unwrap();
  if suite.contains_path(path) {
    Some(suite)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use config::{Suite, SuitePath};
  use util::{PathWrapper};
  use super::{identify};

  fn get_suite(root_path: &str) -> Suite {
    let root = PathWrapper::from_str(root_path);
    let paths = vec![SuitePath::new(&root, "*.txt".to_string())];
    Suite {
      group: "xfoo".to_string(),
      root: root,
      paths: paths,
      executable: "cat".to_string()
    }
  }

  fn get_suites(root_paths: Vec<&str>) -> Vec<Suite> {
    root_paths.iter().map(|root_path| {
      get_suite(*root_path)
    }).collect()
  }

  // Find a single suite by root note that this will fail the task rather then
  // using an option since it's easier and clearer for testing.
  fn suite_by_root<'a>(
    root_path: &str, suites: &'a Vec<Suite>
  ) -> &'a Suite<'a> {

    let root = PathWrapper::from_str(root_path);
    suites.iter().find(|suite| {
      suite.root == root
    }).unwrap()
  }

  #[test]
  fn identify_found_single_root() {
    let suites = get_suites(vec!["/find_me"]);
    let path = Path::new("/find_me/woot.txt");
    assert_eq!(
      identify(&path, &suites).unwrap(), suite_by_root("/find_me", &suites)
    );
  }

  #[test]
  fn identify_found_closest_root() {
    let suites = get_suites(vec!["/find_me/level/1/two", "/find_me/level/1/"]);

    let path = Path::new("/find_me/level/1/woot.txt");
    assert_eq!(
      identify(&path, &suites).unwrap(),
      suite_by_root("/find_me/level/1/", &suites)
    );
  }

  #[test]
  fn identify_found_deepest_root() {
    let suites = get_suites(vec!["/find_me/level/1/two", "/find_me/level/1/"]);

    let path = Path::new("/find_me/level/1/two/woot.txt");
    assert_eq!(
      identify(&path, &suites).unwrap(),
      suite_by_root("/find_me/level/1/two", &suites)
    );
  }

  #[test]
  fn identify_none_file_mismatch() {
    let suites = get_suites(vec!["/foo"]);
    let path = Path::new("/foo/another-thing");
    assert_eq!(identify(&path, &suites), None)
  }

  #[test]
  fn identify_none_root_mismatch() {
    let suites = get_suites(vec!["/bar"]);
    let path = Path::new("/foo/xfoo.txt");
    assert_eq!(identify(&path, &suites), None)
  }
}
