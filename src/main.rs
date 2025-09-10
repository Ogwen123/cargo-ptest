mod run;
mod parse;

use std::process::{Command, Stdio};
use crate::run::run;

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    mod tests;
}
