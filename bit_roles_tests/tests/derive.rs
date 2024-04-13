use bit_roles::BitRole;

#[test]
fn can_derive_checked() {
    #[allow(dead_code)]
    #[derive(Debug, BitRole, Copy, Clone)]
    enum TestRole {
        None = 0,
        One = 1,
    }

    let roles = TestRole::empty();

    assert_eq!(roles.get_value(), 0);
}
