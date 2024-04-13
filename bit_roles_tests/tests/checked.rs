use bit_roles::BitRole;
use std::ops::BitOrAssign;

#[allow(dead_code)]
#[derive(Debug, BitRole, Copy, Clone)]
enum TestRole {
    None = 0,
    One = 1,
    Two = 2,
}

#[test]
fn can_create_an_empty_manager() {
    let manager = TestRole::empty();
    assert_eq!(manager.get_value(), 0);
}

#[test]
fn can_create_a_manager_from_value() {
    let manager = TestRole::from_value(5);
    assert_eq!(manager.get_value(), 5);
}

// Methods

#[test]
fn add_one() {
    let mut manager = TestRole::empty();
    manager.add_one(TestRole::One);

    assert_eq!(manager.get_value(), TestRole::One as usize);
}

#[test]
fn add_all() {
    let mut manager = TestRole::empty();
    manager.add_all(vec![TestRole::One, TestRole::Two]);

    let mut expected = TestRole::One as usize;
    expected.bitor_assign(TestRole::Two as usize);

    assert_eq!(manager.get_value(), expected);
}

#[test]
fn remove_one() {
    let mut manager = TestRole::empty();
    manager.add_one(TestRole::One);

    assert_eq!(manager.get_value(), TestRole::One as usize);

    manager.remove_one(TestRole::One);

    assert_eq!(manager.get_value(), 0);
}

#[test]
fn remove_all() {
    let mut manager = TestRole::empty();
    manager.add_all(vec![TestRole::One, TestRole::Two]);

    let mut expected = TestRole::One as usize;
    expected.bitor_assign(TestRole::Two as usize);

    assert_eq!(manager.get_value(), expected);

    manager.remove_all(vec![TestRole::One, TestRole::Two]);

    assert_eq!(manager.get_value(), 0);
}

#[test]
fn has_one() {
    let mut manager = TestRole::empty();

    assert!(!manager.has_one(TestRole::One));

    manager.add_one(TestRole::One);

    assert!(manager.has_one(TestRole::One));
}

#[test]
fn has_all() {
    let mut manager = TestRole::empty();
    manager.add_one(TestRole::One);

    assert!(!manager.has_all(vec![TestRole::One, TestRole::Two]));

    manager.add_one(TestRole::Two);

    assert!(manager.has_all(vec![TestRole::One, TestRole::Two]));
}

#[test]
fn has_any() {
    let mut manager = TestRole::empty();

    assert!(!manager.has_any(vec![TestRole::One, TestRole::Two]));

    manager.add_one(TestRole::One);

    assert!(manager.has_any(vec![TestRole::One, TestRole::Two]));
}

#[test]
fn not_one() {
    let mut manager = TestRole::empty();

    assert!(manager.not_one(TestRole::One));

    manager.add_one(TestRole::One);

    assert!(!manager.not_one(TestRole::One));
}

#[test]
fn not_all() {
    let mut manager = TestRole::empty();
    manager.add_one(TestRole::One);

    assert!(manager.not_all(vec![TestRole::One, TestRole::Two]));

    manager.add_one(TestRole::Two);

    assert!(!manager.not_all(vec![TestRole::One, TestRole::Two]));
}

#[test]
fn not_any() {
    let mut manager = TestRole::empty();

    assert!(manager.not_any(vec![TestRole::One, TestRole::Two]));

    manager.add_one(TestRole::One);

    assert!(!manager.not_any(vec![TestRole::One, TestRole::Two]));
}

#[test]
fn equality() {
    let mut m1 = TestRole::empty();
    let mut m2 = TestRole::empty();

    m1.add_one(TestRole::One);
    m2.add_one(TestRole::One);

    assert_eq!(m1, m2);
}
