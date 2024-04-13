//! # Bit Roles
//!
//! [![Latest Version](https://img.shields.io/crates/v/bit_roles.svg)](https://crates.io/crates/bit_roles)
//! [![Rust Documentation](https://docs.rs/bit_roles/badge.svg)](https://docs.rs/bit_roles)
//!
//! This crate enables you to implement granular role and permission management
//! based on bit flags.
//!
//! You can derive the [BitRole] trait for your role enum. It ensures
//! compile-time validation for enum discriminants. Ensure you specify a
//! discriminant for every enum variant; it must be either zero or a power of
//! two. Also, remember to derive the [Copy] and [Clone] traits for your enum.
//!
//! If you need a manager without compile-time checks, it's also exported as
//! [BitRoleUnchecked] trait. This is useful if you want to use raw integer
//! values for roles or have a complex role enum definition. You will need to
//! implement the `Into<usize>` trait for your role enum, along with deriving
//! the [Copy] and [Clone] traits for it.
//!
//! # Examples
//!
//! ```
//! use bit_roles::BitRole;
//!
//! #[derive(Debug)]
//! struct User {
//!     permissions: usize,
//! }
//!
//! #[derive(Debug, BitRole, Copy, Clone)]
//! enum Permission {
//!     None = 0,
//!     SendMessage = 1,
//!     EditMessage = 2,
//! }
//!
//! let mut permissions = Permission::empty();
//! permissions.add_one(Permission::SendMessage);
//!
//! let user = User {
//!     permissions: permissions.get_value(),
//! };
//!
//! // Checking if the user can edit messages.
//! let manager = Permission::from_value(user.permissions);
//! let can_edit_messages = manager.has_one(Permission::EditMessage); // `false`
//! ```

#![forbid(unsafe_code)]
#![warn(future_incompatible, missing_docs)]

mod checked;
mod error;
mod role_value;
mod unchecked;
mod utils;

pub use bit_roles_macros::{
    BitRole,
    BitRoleUnchecked,
};
pub use checked::*;
pub use error::RoleError;
pub use role_value::RoleValue;
pub use unchecked::*;
pub use utils::is_validate_role;

/// The role variant trait. All role enums must implement this trait.
pub trait RoleVariant: Into<usize> + Copy {}
