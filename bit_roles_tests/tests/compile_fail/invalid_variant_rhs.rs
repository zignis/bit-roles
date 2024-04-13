use bit_roles::BitRole;

// Enum with invalid values.
#[derive(Debug, BitRole, Copy, Clone)]
enum RoleOne {
    One = 0,
    Two = 5,
}

// Enum with signed values.
#[derive(Debug, BitRole, Copy, Clone)]
enum RoleTwo {
    One = 0,
    Two = -3,
}

fn main() {}
