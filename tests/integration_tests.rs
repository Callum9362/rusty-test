use unit_tests;

#[test]
fn test_add_integration() {
    assert_eq!(unit_tests::add(4, 6), 10);
}

#[test]
fn test_divide_integration() {
    assert_eq!(unit_tests::divide(9, 3), Ok(3));
}
