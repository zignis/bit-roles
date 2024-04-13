# Bit Roles

[![Latest Version](https://img.shields.io/crates/v/bit_roles.svg)](https://crates.io/crates/bit_roles)
[![Rust Documentation](https://docs.rs/bit_roles/badge.svg)](https://docs.rs/bit_roles)

This crate enables you to implement granular role and permission management based on bit flags.

<table>
    <tr>
        <th> Using discrete fields </th>
        <th> Using bit roles </th>
    </tr>
<tr>
<td>

```rust
#[derive(Debug)]
struct User {
    can_send_message: bool,
    can_edit_message: bool
}

fn main() {
    let user = User {
        can_send_message: true,
        can_edit_message: false
    };
}
```

</td>
<td>

```rust
use bit_roles::BitRole;

#[derive(Debug)]
struct User {
    permissions: usize
}

#[derive(Debug, BitRole, Copy, Clone)]
enum Permission {
    None = 0,
    SendMessage = 1,
    EditMessage = 2,
}

fn main() {
    let mut permissions = Permission::empty();
    permissions.add_one(Permission::SendMessage);

    let user = User {
        permissions: permissions.get_value()
    };
}
```

</td>
</tr>
</table>

## Getting started

Add `bit_roles` to your project:

```shell
cargo add bit_rols
```

## Usage

You can derive the `BitRole` trait for your role enum. It ensures compile-time validation for enum discriminants. Ensure
you specify a discriminant for every enum variant; it must be either zero or a power of two. Also, remember to derive
the `Copy` and `Clone` traits for your enum.

If you need a manager without compile-time checks, it's also exported as `BitRoleUnchecked` trait. This is useful if you
want to use raw integer values for roles or have a complex role enum definition. You will need to implement
the `Into<usize>` trait for your role enum, along with deriving the `Copy` and `Clone` traits for it.
