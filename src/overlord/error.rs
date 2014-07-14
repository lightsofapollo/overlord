#[deriving(Show)]
pub struct OverlordError {
  pub message: String
}

impl OverlordError {
  pub fn human_error(self) -> String {
    // TODO: Make this look amazing.
    self.message
  }

  pub fn new(message: String) -> OverlordError {
    OverlordError{message: message}
  }
}

pub type OverlordResult<T> = Result<T, OverlordError>;
