use bit_roles::BitRoleUnchecked;

#[test]
fn can_derive_unchecked() {
    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone)]
    enum Nested {
        One,
        Two,
    }

    #[allow(dead_code)]
    #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    enum TestRole {
        None,
        One,
        Two(Nested),
    }

    impl From<TestRole> for usize {
        fn from(val: TestRole) -> Self {
            match val {
                TestRole::None => 0,
                TestRole::One => 1,
                TestRole::Two(variant) => match variant {
                    Nested::One => 2,
                    Nested::Two => 4,
                },
            }
        }
    }

    let roles = TestRole::empty();

    assert_eq!(roles.get_value(), 0);
}
