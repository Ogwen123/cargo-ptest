use cargo_ptest::display::{colour, Colour};

#[test]
fn colour_test_green() {
    assert_eq!(colour(Colour::GREEN, ""), String::from("test"))
}

#[test]
fn colour_test_red() {
    assert_eq!(colour(Colour::RED, ""), String::from("test"))
}