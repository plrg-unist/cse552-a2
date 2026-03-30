#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate either;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hash;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

pub mod analysis;
pub mod expr;
pub mod utils;

#[cfg(test)]
mod tests;
