#[test]
fn succeed() {
    assert_eq!(1, 1)
}

#[test]
fn panic() {
    assert_eq!(2, 1)
}

#[test]
#[should_panic]
#[ignore]
fn should_panic_and_does() {
    assert_eq!(2, 1)
}

#[test]
#[should_panic]
fn should_panic_and_doesnt() {
    assert_eq!(1, 1)
}