macro_rules! parse_error {
    ($($args:tt)*) => {
        Err(ParseError { error: format!($($args)*) })
    };
}

struct ParseError {
    error: String
}

pub fn parse(output: String) -> Result<String, ParseError> {
    Ok(String::new())
}