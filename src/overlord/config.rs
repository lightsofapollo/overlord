// This module contains the he canonical interfaaces used by all internal
// operations in overlord it may or may not conform to the same format as the
// user facing interchange format.

// All operations stem from the "suite" configuration.
pub struct Suite<'a> {
  /// Name of the "group" this suite belongs to.
  pub group: String,

  /// The Path in which to conduct matching of files, etc...
  pub root: Path,

  /// A list of "paths" (may also be globs) for the suite.
  pub paths: Vec<String>,

  /// The executable to use to run files for this suite.
  pub executable: String
}
