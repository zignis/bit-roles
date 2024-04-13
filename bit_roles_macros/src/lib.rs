//! # Bit Roles Macros
//!
//! This crate enables you to implement granular role and permission management
//! based on bit flags.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input,
    Data,
    DeriveInput,
    Expr,
    Lit,
    Variant,
};

/// Returns a new [syn::Error] with the provided error message.
///
/// * `message` - The error message.
fn throw_error(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

/// Validates the discriminant of an enum variant.
///
/// * `variant` - The enum variant.
/// * `enum_name` - The literal name of the enum.
fn validate_enum_variant(variant: Variant, enum_name: &str) -> Result<(), syn::Error> {
    let variant_name = variant.ident;
    let (_, expression) = variant.discriminant.ok_or(throw_error(
        format!(
            "`{variant_name}` in the `{enum_name}` enum must have a hard-coded discriminant value"
        )
        .as_str(),
    ))?;

    match expression {
        Expr::Lit(expr) => match expr.lit {
            Lit::Int(value) => {
                let value = value.base10_parse::<usize>().map_err(|_| {
                    throw_error(
                        format!("[`{variant_name}`]: cannot parse `{value}` as `usize`").as_str(),
                    )
                })?;

                if value != 0 && !value.is_power_of_two() {
                    Err(throw_error(
                        format!("[`{variant_name}`]: `{value}` is neither zero nor a power of two")
                            .as_str(),
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Err(throw_error(
                format!(
                    "`{variant_name}` in the `{enum_name}` enum must have an integer discriminant"
                )
                .as_str(),
            )),
        },
        _ => Err(throw_error(
            format!(
                "`{variant_name}` in the `{enum_name}` enum must have a literal RHS expression"
            )
            .as_str(),
        )),
    }
}

/// Bit role manager with compile-time value checking. Useful when you have
/// a simple role enum definition and do not wish to work with raw integer role
/// values. Each variant of your role enum must return a valid role value that
/// is either zero or a power of two. Your role enum must also derive the [Copy]
/// and [Clone] traits.
///
/// Check the `BitRoleUnchecked` variant if you need to work with raw integer
/// role values or you have a complex role enum definition.
///
/// # Examples
///
/// Using simple role enum definitions.
///
/// ```
/// use bit_roles::BitRole;
///
/// #[derive(Debug, BitRole, Copy, Clone)]
/// enum Permission {
///     None = 0,
///     SendMessage = 1,
///     EditMessage = 2,
/// }
///
/// let mut roles = Permission::empty();
///
/// // Add a single role to the manager.
/// roles.add_one(Permission::SendMessage);
///
/// assert!(roles.has_one(Permission::SendMessage));
/// ```
///
/// A compile-time error will be generated if any of the enum variant returns
/// value that is neither zero nor a power of two.
///
/// ```compile_fail
/// use bit_roles::BitRole;
///
/// // This should not compile.
/// #[derive(Debug, BitRole, Copy, Clone)]
/// enum Permission {
///     None = 0,
///     InvalidRole = 5,
/// }
/// ```
#[proc_macro_derive(BitRole)]
pub fn derive_bit_role(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Enum(value) => {
            let name = input.ident;
            let enum_name = name.to_string();
            let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

            // Validate enum variant discriminants.
            for variant in value.variants.clone() {
                match validate_enum_variant(variant, &enum_name) {
                    Ok(_) => {}
                    Err(err) => return err.to_compile_error().into(),
                }
            }

            let expanded = quote! {
                use bit_roles::BitRoleImpl;
                use std::marker::PhantomData;

                impl #impl_generics Into<usize> for #name #ty_generics #where_clause {
                    fn into(self) -> usize {
                        self as usize
                    }
                }

                impl #impl_generics bit_roles::RoleVariant for #name #ty_generics #where_clause {}

                impl #impl_generics BitRoleImpl<#name> for #name #ty_generics #where_clause {
                    fn empty() -> bit_roles::RoleManager<#name> {
                        bit_roles::RoleManager(0, PhantomData)
                    }

                    fn from_value(value: usize) -> bit_roles::RoleManager<#name> {
                        bit_roles::RoleManager(value, PhantomData)
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => throw_error("This macro can only be used with enums.")
            .to_compile_error()
            .into(),
    }
}

/// Bit role manager without value checking. Useful when you want to use raw
/// integer role values or you have a complex role enum definition. This
/// requires you to implement the `Into<usize>` trait for your role enum
/// yourself, and each variant must return a valid role value that is either
/// zero or a power of two. Your role enum must also derive the [Copy] and
/// [Clone] traits.
///
/// # Examples
///
/// Using raw integer values for role management.
///
/// ```
/// use bit_roles::{
///     BitRoleUnchecked,
///     RoleValue,
/// };
///
/// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
/// enum Permission {
///     None = 0,
///     SendMessage = 1,
///     EditMessage = 2,
/// }
///
/// impl Into<usize> for Permission {
///     fn into(self) -> usize {
///         self as usize
///     }
/// }
///
/// let mut roles = Permission::empty();
///
/// // Use a raw integer value for role.
/// roles
///     .try_add_one(RoleValue::Raw(2))
///     .expect("invalid role value");
///
/// assert!(roles.has_one(Permission::EditMessage));
/// ```
///
/// Using complex role enum definitions.
///
/// ```
/// use bit_roles::{
///     BitRoleUnchecked,
///     RoleValue,
/// };
///
/// #[derive(Debug, Copy, Clone)]
/// enum SendMessagePermission {
///     ToEveryone,
///     ToFriends,
/// }
///
/// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
/// enum Permission {
///     None,
///     SendMessage(SendMessagePermission),
///     EditMessage,
/// }
///
/// impl Into<usize> for Permission {
///     fn into(self) -> usize {
///         match self {
///             Permission::None => 0,
///             Permission::SendMessage(variant) => match variant {
///                 SendMessagePermission::ToEveryone => 1,
///                 SendMessagePermission::ToFriends => 2,
///             },
///             Permission::EditMessage => 4,
///         }
///     }
/// }
///
/// let mut roles = Permission::empty();
///
/// // Add a complex role to the manager.
/// roles
///     .try_add_one(RoleValue::Role(Permission::SendMessage(
///         SendMessagePermission::ToEveryone,
///     )))
///     .expect("invalid role value");
///
/// assert!(roles.has_one(Permission::SendMessage(SendMessagePermission::ToEveryone)));
/// ```
#[proc_macro_derive(BitRoleUnchecked)]
pub fn derive_bit_role_unchecked(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        Data::Enum(_) => {
            let expanded = quote! {
                use bit_roles::BitRoleUncheckedImpl;
                use std::marker::PhantomData;

                impl #impl_generics bit_roles::RoleVariant for #name #ty_generics #where_clause {}

                impl #impl_generics BitRoleUncheckedImpl<#name> for #name #ty_generics #where_clause {
                    fn empty() -> bit_roles::RoleManagerUnchecked<#name> {
                        bit_roles::RoleManagerUnchecked(0, PhantomData)
                    }

                    fn from_value(value: usize) -> bit_roles::RoleManagerUnchecked<#name> {
                        bit_roles::RoleManagerUnchecked(value, PhantomData)
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => throw_error("This macro can only be used with enums.")
            .to_compile_error()
            .into(),
    }
}
