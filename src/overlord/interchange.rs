// This module contains the "interchange" format intended to be used by
// consumers this _should_ be a direct mapping to the actual format used (json,
// toml, yaml, whatever...).

/// The top level structure is the "manifest" itself.
#[deriving(Decodable)]
pub struct Manifest {
  /// Manifests may contain references to other manfiests.
  pub manifests: Option<Vec<Path>>,

  /// Individual suites inside of the primary manifest.
  pub suites: Vec<ManifestSuite>
}

/// Individual suites inside of the manifest.
#[deriving(Decodable)]
pub struct ManifestSuite {
  /// The group this suite belongs to.
  pub group: String,

  /// The paths/globs to tests in this suite.
  pub paths: Vec<String>,

  /// Executable used to run files in this suite.
  pub executable: String
}
