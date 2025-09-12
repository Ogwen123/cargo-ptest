use std::collections::HashMap;
use crate::parse::ResultOption;

pub enum Colour {
    GREEN,
    RED,
    ORANGE
}

pub fn colour<T>(colour: Colour, text: T) -> String
where T: ToString {
    return String::from("test")
}

pub fn display(tree: HashMap<String, ResultOption>) {
    
}