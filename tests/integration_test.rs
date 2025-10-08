use cargo_ptest::display::{Colour, Display};

#[test]
fn colour_test_green() {
    assert_eq!(Display::colour(Colour::GREEN, ""), String::from("test"))
}

#[test]
fn colour_test_red() {
    assert_eq!(Display::colour(Colour::RED, ""), String::from("test"))
}
