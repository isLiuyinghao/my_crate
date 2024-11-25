use my_crate::{add, subtract};

#[test]
fn test_integration_add() {
    assert_eq!(add(5, 6), 11);
}

#[test]
fn test_integration_subtract() {
    assert_eq!(subtract(10, 3), 7);
}
