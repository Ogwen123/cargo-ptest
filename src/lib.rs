//! You can install this crate to use it as a binary or add it to a project to use it as a library
//! * As a library it can run the cargo test command and parse the output into a list of [ParsedTestGroups].
//! * As a binary it can be used with `cargo ptest`, see the readme for more information about using it as a binary
//!
//! This project was inspired by [cargo-pretty-test](https://crates.io/crates/cargo-pretty-test)

pub mod config;
pub mod display;
mod logger;
pub mod parse;
pub mod run;
