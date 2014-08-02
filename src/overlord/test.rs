use std::os;
use util::{PathWrapper};

pub fn assert_path_wrapper_eq(expected: &Path, actual: &PathWrapper) {
  assert_path_eq(expected, &os::make_absolute(actual.get()))
}

pub fn assert_path_eq(expected: &Path, actual: &Path) {
  let abs_expected = os::make_absolute(expected);
  let abs_actual = os::make_absolute(actual);

  assert!(
    abs_expected == abs_actual,
    format!(
      "expected '{}' to equal {}", abs_expected.display(), abs_actual.display()
    )
  );
}
