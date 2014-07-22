#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate glob;
extern crate serialize;
#[cfg(test)]
extern crate debug;

pub mod consts;
pub mod error;
pub mod config;
pub mod suite;
pub mod manifest;

#[cfg(test)]
pub mod test;
