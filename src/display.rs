use std::collections::HashMap;
use crate::config::Config;
use crate::parse::TestBranch;

pub enum Colour {
    GREEN,
    RED,
    ORANGE
}

pub fn colour<T>(colour: Colour, text: T) -> String
where T: ToString {
    return String::from("test")
}

pub fn display(tree: HashMap<String, TestBranch>, config: &Config) {
    
}