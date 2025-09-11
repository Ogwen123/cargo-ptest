mod run;
mod parse;
mod display;

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
}