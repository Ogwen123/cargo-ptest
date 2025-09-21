mod run;
mod parse;
mod display;
mod config;

use crate::run::run;

fn main() {
    match run() {
        Ok(_) => {},
        Err(err) => println!("{}", err)
    }
}

#[cfg(test)]
mod tests {
    mod tests;
    mod tests2;
}