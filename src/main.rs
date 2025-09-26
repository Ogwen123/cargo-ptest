mod config;
mod display;
mod parse;
mod run;

use crate::run::run;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    mod tests;
    mod tests2;
}
