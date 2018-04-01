#[test]
fn rpn_test() {
    use super::rpn;
    assert_eq!(rpn("15 7 1 1 + - / 3 * 2 1 1 + + -".to_string()), Ok(5));
}
