// Generic utils for rust and not specific to core Overlord functionality... The
// intent is these utilities will either be replaced, moved or become useless...
use std::fmt::{Show, Formatter, FormatError};

// A single layer of indirection around a path simply so we can
#[deriving(PartialEq)]
pub struct PathWrapper {
  value: Path
}

impl PathWrapper {
  pub fn get(&self) -> &Path {
    return &self.value
  }

  pub fn from_str(path: &str) -> PathWrapper {
    PathWrapper { value: Path::new(path) }
  }

  pub fn new(path: Path) -> PathWrapper {
    PathWrapper { value: path }
  }
}

// Important bit of PathSrc here is the ability to "show" the path this is
// hugely useful for tests and to display errors, etc..
impl Show for PathWrapper {
  fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
    self.value.display().fmt(f)
  }
}
