use cargo_ptest::display::{colour, Colour};

#[test]
fn colour_test() {
    assert_eq!(colour(Colour::GREEN, ""), String::from("test"))
}
