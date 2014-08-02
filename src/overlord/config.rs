// This module contains the he canonical interfaaces used by all internal
// operations in overlord it may or may not conform to the same format as the
// user facing interchange format.
use util::{PathWrapper};
use glob::{Pattern, MatchOptions};
use std::fmt::{FormatError, Formatter, Show};
use std::cmp::{PartialEq};

// A suite "path" is a level of indirection around a glob pattern with show
// funcitonality and a constructor geared towards overlord specific path
// matching.
pub struct SuitePath {
  // Actual glob pattern used to match the path.
  pub pattern: Pattern,

  // Human readable path
  pub path: String
}

impl SuitePath {
  pub fn new(root: &PathWrapper, path: String) -> SuitePath {
    // This is probably a hack but is the most convient way to join the
    // strings. A potential problem here is if the "glob" pattern somehow
    // messes up the join. Another issue is the use of `..` in paths...
    let joined = root.get().join(Path::new(path.clone()));
    let joined_str = joined.as_str().unwrap();
    let joined_path = joined_str.to_string();

    SuitePath {
      // unwrap is somewhat lazy here but doing this "correctly" is more work
      // and make the signature uglier.
      pattern: Pattern::new(joined_str),
      path: joined_path
    }
  }
}

// For tests we need the ability to assert equality...
impl PartialEq for SuitePath {
  fn eq(&self, other: &SuitePath) -> bool {
    self.path == other.path
  }
}

// For nice output we still show the same value as we get from the raw
// configuration
impl Show for SuitePath {
  fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
    self.path.fmt(f)
  }
}

// All operations stem from the "suite" configuration.
#[deriving(PartialEq, Show)]
pub struct Suite<'a> {
  /// Name of the "group" this suite belongs to.
  pub group: String,

  /// The Path in which to conduct matching of files, etc...
  pub root: PathWrapper,

  /// A list of "paths" (may also be globs) for the suite.
  pub paths: Vec<SuitePath>,

  /// The executable to use to run files for this suite.
  pub executable: String
}


impl<'a> Suite<'a> {
  /// Determine if a given file matches any of the "path" pattern rules in this
  /// suite.
  pub fn contains_path(&self, path: &Path) -> bool {
    // These options are used so frequently it would make sense to stash them
    // somewhere higher up likely...
    let match_options = MatchOptions {
      case_sensitive: true,
      // Do not match into subdirectories implicitly
      require_literal_separator: true,
      require_literal_leading_dot: false
    };


    for suite_path in self.paths.iter() {
      let matches = suite_path.pattern.matches_path_with(path, match_options);
      if matches {
        return true
      }
    }
    return false
  }
}

#[cfg(test)]
mod tests {
  use util::{PathWrapper};
  use super::{SuitePath, Suite};

  fn get_suite<'a>() -> Suite<'a> {
    let root = PathWrapper::from_str("/foo");
    let paths = vec![
      SuitePath::new(&root, "*_test.txt".to_string()),
      SuitePath::new(&root, "nested/bar/*_test.txt".to_string()),
    ];

    Suite {
      group: "xfoo".to_string(),
      root: root,
      paths: paths,
      executable: "cat".to_string()
    }
  }

  #[test]
  fn new_suite_path() {
    let subject =
      SuitePath::new(&PathWrapper::from_str("/User/foo"), "*.txt".to_string());

    // Quick sanity check to ensure pattern matching is working from the correct
    // root.
    assert!(subject.pattern.matches("/User/foo/xfoo.txt"));
    assert_eq!(subject.pattern.matches("/User/foo/xfoo.js"), false);
    assert_eq!(subject.pattern.matches("xfoo.txt"), false);
  }

  #[test]
  fn suite_path_matches_none() {
    let suite = get_suite();
    assert_eq!(
      suite.contains_path(&Path::new("/foo/other/file_test.txt")), false
    );
  }

  #[test]
  fn suite_path_matches() {
    let suite = get_suite();
    assert!(suite.contains_path(&Path::new("/foo/1_test.txt")));
    assert!(suite.contains_path(&Path::new("/foo/2_test.txt")));
    assert!(suite.contains_path(&Path::new("/foo/nested/bar/1_test.txt")));
    assert!(suite.contains_path(&Path::new("/foo/nested/bar/2_test.txt")));
  }
}
