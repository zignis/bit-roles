use crate::{
    utils::{
        is_validate_role,
        negate,
    },
    RoleError,
    RoleValue,
    RoleVariant,
};
use std::{
    marker::PhantomData,
    ops::{
        BitAnd,
        BitAndAssign,
        BitOrAssign,
    },
};

/// Unchecked bit role trait. Implements [RoleManagerUnchecked] for a role enum.
pub trait BitRoleUncheckedImpl<T> {
    /// Creates a new [RoleManagerUnchecked] instance with the default value.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// // Create an empty manager instance.
    /// let mut roles = MyRole::empty();
    ///
    /// assert_eq!(roles.get_value(), 0);
    /// ```
    fn empty() -> RoleManagerUnchecked<T>;
    /// Creates a new [RoleManagerUnchecked] instance with the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// // Create a manager instance with the provided value.
    /// let mut roles = MyRole::from_value(5);
    ///
    /// assert_eq!(roles.get_value(), 5);
    /// ```
    ///
    /// * `value` - The value for the manager.
    fn from_value(value: usize) -> RoleManagerUnchecked<T>;
}

/// The unchecked role manager. Typically used when you need to use raw
/// integer role values or have complex enum definitions.
#[derive(Debug)]
pub struct RoleManagerUnchecked<T>(pub usize, pub PhantomData<T>);

impl<T> RoleManagerUnchecked<T>
where
    T: RoleVariant,
{
    /// Validates the magnitude of the role value.
    ///
    /// * `role` - The role value to validate.
    fn validate_role(&self, role: RoleValue<T>) -> Result<usize, RoleError> {
        let mag: usize = role.into();

        is_validate_role(mag)
            .then_some(mag)
            .ok_or(RoleError::InvalidRole(mag))
    }

    /// Converts a vector of roles to a vector of equivalent [RoleValue]
    /// variants.
    ///
    /// * `roles` - The roles to convert.
    fn to_role_values(&self, roles: Vec<T>) -> Vec<RoleValue<T>> {
        roles.into_iter().map(RoleValue::Role).collect::<Vec<_>>()
    }

    /// Validates and adds a single role value to the manager instance. This is
    /// a non-panicking equivalent of the [add_one] method.
    ///
    /// [add_one]: RoleManagerUnchecked::add_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add a new role to the manager.
    /// roles
    ///     .try_add_one(RoleValue::Role(MyRole::Staff))
    ///     .expect("invalid role");
    ///
    /// assert!(roles.has_one(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role value to add to the manager.
    pub fn try_add_one(&mut self, role: RoleValue<T>) -> Result<&mut Self, RoleError> {
        let value = self.validate_role(role)?;
        self.0.bitor_assign(value);

        Ok(self)
    }

    /// Validates and adds multiple role values to the manager instance. This is
    /// a non-panicking equivalent of the [add_all] method.
    ///
    /// [add_all]: RoleManagerUnchecked::add_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add multiple new roles to the manager.
    /// roles
    ///     .try_add_all(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(roles.has_all(vec![MyRole::Staff, MyRole::Member]));
    /// ```
    ///
    /// * `role` - The role values to add to the manager.
    pub fn try_add_all(&mut self, roles: Vec<RoleValue<T>>) -> Result<&mut Self, RoleError> {
        for role in roles {
            self.try_add_one(role)?;
        }

        Ok(self)
    }

    /// Validates and removes a single role value from the manager instance.
    /// This is a non-panicking equivalent of the [remove_one] method.
    ///
    /// [remove_one]: RoleManagerUnchecked::remove_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Remove a role from the manager.
    /// roles
    ///     .try_remove_one(RoleValue::Role(MyRole::Staff))
    ///     .expect("invalid role");
    ///
    /// assert!(roles.not_one(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role value to remove from the manager.
    pub fn try_remove_one(&mut self, role: RoleValue<T>) -> Result<&mut Self, RoleError> {
        let value = self.validate_role(role)?;
        self.0.bitand_assign(!value);

        Ok(self)
    }

    /// Validates and removes multiple role values from the manager instance.
    /// This is a non-panicking equivalent of the [remove_all] method.
    ///
    /// [remove_all]: RoleManagerUnchecked::remove_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Remove multiple roles from the manager.
    /// roles
    ///     .try_remove_all(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(roles.not_all(vec![MyRole::Staff, MyRole::Member]));
    /// ```
    ///
    /// * `role` - The role values to remove from the manager.
    pub fn try_remove_all(&mut self, roles: Vec<RoleValue<T>>) -> Result<&mut Self, RoleError> {
        for role in roles {
            self.try_remove_one(role)?;
        }

        Ok(self)
    }

    /// Validates and checks whether a single role is assigned to the manager
    /// instance. This is a non-panicking equivalent of the [has_one] method.
    ///
    /// [has_one]: RoleManagerUnchecked::has_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// // Create a role manager with initial `Staff` role.
    /// let mut roles = MyRole::from_value(MyRole::Staff.into());
    ///
    /// // Check if the manager has a single role.
    /// let has_role = roles
    ///     .try_has_one(RoleValue::Role(MyRole::Staff))
    ///     .expect("invalid role");
    ///
    /// assert!(has_role);
    /// ```
    ///
    /// * `role` - The role value to check against the manager.
    pub fn try_has_one(&self, role: RoleValue<T>) -> Result<bool, RoleError> {
        let value = self.validate_role(role)?;
        Ok(self.0.bitand(value) != 0)
    }

    /// Validates and checks whether each of the roles is assigned to the
    /// manager instance. This is a non-panicking equivalent of the [has_all]
    /// method.
    ///
    /// [has_all]: RoleManagerUnchecked::has_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add multiple roles to the role manager.
    /// roles.add_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// // Check if the manager has all the provided roles.
    /// let has_roles = roles
    ///     .try_has_all(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(has_roles);
    /// ```
    ///
    /// * `role` - The role values to check against the manager.
    pub fn try_has_all(&self, roles: Vec<RoleValue<T>>) -> Result<bool, RoleError> {
        let mut flag = false;

        for role in roles {
            flag = self.try_has_one(role)?;
        }

        Ok(flag)
    }

    /// Validates and checks whether any one of the roles is assigned to the
    /// manager instance. This is a non-panicking equivalent of the [has_any]
    /// method.
    ///
    /// [has_any]: RoleManagerUnchecked::has_any
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Create a role manager with initial `Staff` role.
    /// let mut roles = MyRole::from_value(MyRole::Staff.into());
    ///
    /// // Check if the manager has any of the provided roles.
    /// let has_roles = roles
    ///     .try_has_any(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(has_roles);
    /// ```
    ///
    /// * `role` - The role values to check against the manager.
    pub fn try_has_any(&self, roles: Vec<RoleValue<T>>) -> Result<bool, RoleError> {
        let mut flag = false;

        for role in roles {
            if self.try_has_one(role)? {
                flag = true;
                break;
            }
        }

        Ok(flag)
    }

    /// Validates and checks whether a single role is not assigned to the
    /// manager instance. This is a non-panicking equivalent of the [not_one]
    /// method.
    ///
    /// [not_one]: RoleManagerUnchecked::not_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have a single role.
    /// let does_not_have_role = roles
    ///     .try_not_one(RoleValue::Role(MyRole::Staff))
    ///     .expect("invalid role");
    ///
    /// assert!(does_not_have_role);
    /// ```
    ///
    /// * `role` - The role value to check against the manager.
    pub fn try_not_one(&self, role: RoleValue<T>) -> Result<bool, RoleError> {
        self.try_has_one(role).map(negate)
    }

    /// Validates and checks whether each of the roles is not assigned to the
    /// manager instance. This is a non-panicking equivalent of the [not_all]
    /// method.
    ///
    /// [not_all]: RoleManagerUnchecked::not_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have all of the provided roles.
    /// let does_not_have_roles = roles
    ///     .try_not_all(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(does_not_have_roles);
    /// ```
    ///
    /// * `role` - The role values to check against the manager.
    pub fn try_not_all(&self, roles: Vec<RoleValue<T>>) -> Result<bool, RoleError> {
        self.try_has_all(roles).map(negate)
    }

    /// Validates and checks whether any of the roles is not assigned to the
    /// manager instance. This is a non-panicking equivalent of the [not_any]
    /// method.
    ///
    /// [not_any]: RoleManagerUnchecked::not_any
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have any of the provided roles.
    /// let does_not_have_roles = roles
    ///     .try_not_any(vec![
    ///         RoleValue::Role(MyRole::Staff),
    ///         RoleValue::Role(MyRole::Member),
    ///     ])
    ///     .expect("invalid roles");
    ///
    /// assert!(does_not_have_roles);
    /// ```
    ///
    /// * `role` - The role values to check against the manager.
    pub fn try_not_any(&self, roles: Vec<RoleValue<T>>) -> Result<bool, RoleError> {
        self.try_has_any(roles).map(negate)
    }

    /// Adds a single role to the manager instance. Panics if the role is
    /// invalid. Use [try_add_one] as a non-panicking equivalent.
    ///
    /// [try_add_one]: RoleManagerUnchecked::try_add_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add a new role to the manager.
    /// roles.add_one(MyRole::Staff);
    ///
    /// assert!(roles.has_one(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role to add to the manager.
    pub fn add_one(&mut self, role: T) -> &mut Self {
        self.try_add_one(RoleValue::Role(role))
            .expect("`role` is invalid")
    }

    /// Adds multiple roles to the manager instance. Panics if any of the roles
    /// is invalid. Use [try_add_all] as a non-panicking equivalent.
    ///
    /// [try_add_all]: RoleManagerUnchecked::try_add_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add multiple new roles to the manager.
    /// roles.add_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(roles.has_all(vec![MyRole::Staff, MyRole::Member]));
    /// ```
    ///
    /// * `roles` - The roles to add to the manager.
    pub fn add_all(&mut self, roles: Vec<T>) -> &mut Self {
        self.try_add_all(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Removes a single role from the manager instance. Panics if the role is
    /// invalid. Use [try_remove_one] as a non-panicking equivalent.
    ///
    /// [try_remove_one]: RoleManagerUnchecked::try_remove_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Remove a role from the manager.
    /// roles.remove_one(MyRole::Staff);
    ///
    /// assert!(roles.not_one(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role to remove from the manager.
    pub fn remove_one(&mut self, role: T) -> &mut Self {
        self.try_remove_one(RoleValue::Role(role))
            .expect("`role` is invalid")
    }

    /// Removes multiple roles from the manager instance. Panics if any of the
    /// roles is invalid. Use [try_remove_all] as a non-panicking
    /// equivalent.
    ///
    /// [try_remove_all]: RoleManagerUnchecked::try_remove_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Remove multiple roles from the manager.
    /// roles.remove_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(roles.not_all(vec![MyRole::Staff, MyRole::Member]));
    /// ```
    ///
    /// * `roles` - The roles to remove from the manager.
    pub fn remove_all(&mut self, roles: Vec<T>) -> &mut Self {
        self.try_remove_all(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Checks whether a single role is assigned to the manager instance. Panics
    /// if the role is invalid. Use [try_has_one] as a non-panicking equivalent.
    ///
    /// [try_has_one]: RoleManagerUnchecked::try_has_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// // Create a role manager with initial `Staff` role.
    /// let mut roles = MyRole::from_value(MyRole::Staff.into());
    ///
    /// // Check if the manager has a single role.
    /// let has_role = roles.has_one(MyRole::Staff);
    ///
    /// assert!(has_role);
    /// ```
    ///
    /// * `role` - The role to check against the manager.
    pub fn has_one(&self, role: T) -> bool {
        self.try_has_one(RoleValue::Role(role))
            .expect("`role` is invalid")
    }

    /// Checks whether each of the roles is assigned to the manager instance.
    /// Panics if any of the roles is invalid. Use [try_has_all] as a
    /// non-panicking equivalent.
    ///
    /// [try_has_all]: RoleManagerUnchecked::try_has_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Add multiple roles to the role manager.
    /// roles.add_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// // Check if the manager has all the provided roles.
    /// let has_roles = roles.has_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(has_roles);
    /// ```
    ///
    /// * `roles` - The roles to check against the manager.
    pub fn has_all(&self, roles: Vec<T>) -> bool {
        self.try_has_all(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Checks whether any one of the roles is assigned to the manager instance.
    /// Panics if any of the roles is invalid. Use [try_has_any] as a
    /// non-panicking equivalent.
    ///
    /// [try_has_any]: RoleManagerUnchecked::try_has_any
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Create a role manager with initial `Staff` role.
    /// let mut roles = MyRole::from_value(MyRole::Staff.into());
    ///
    /// // Check if the manager has any of the provided roles.
    /// let has_roles = roles.has_any(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(has_roles);
    /// ```
    ///
    /// * `roles` - The roles to check against the manager.
    pub fn has_any(&self, roles: Vec<T>) -> bool {
        self.try_has_any(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Checks whether a single role is not assigned to the manager instance.
    /// Panics if the role is invalid. Use [try_not_one] as a non-panicking
    /// equivalent.
    ///
    /// [try_not_one]: RoleManagerUnchecked::try_not_one
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have a single role.
    /// let does_not_have_role = roles.not_one(MyRole::Staff);
    ///
    /// assert!(does_not_have_role);
    /// ```
    ///
    /// * `role` - The role to check against the manager.
    pub fn not_one(&self, role: T) -> bool {
        self.try_not_one(RoleValue::Role(role))
            .expect("`role` is invalid")
    }

    /// Checks whether each of the roles is not assigned to the manager
    /// instance. Panics if any of the roles is invalid. Use [try_not_all] as a
    /// non-panicking equivalent.
    ///
    /// [try_not_all]: RoleManagerUnchecked::try_not_all
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have all of the provided roles.
    /// let does_not_have_roles = roles.not_all(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(does_not_have_roles);
    /// ```
    ///
    /// * `roles` - The roles to check against the manager.
    pub fn not_all(&self, roles: Vec<T>) -> bool {
        self.try_not_all(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Checks whether any of the roles is not assigned to the manager instance.
    /// Panics if any of the roles is invalid. Use [try_not_any] as a
    /// non-panicking equivalent.
    ///
    /// [try_not_any]: RoleManagerUnchecked::try_not_any
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let mut roles = MyRole::empty();
    ///
    /// // Check if the manager does not have any of the provided roles.
    /// let does_not_have_roles = roles.not_any(vec![MyRole::Staff, MyRole::Member]);
    ///
    /// assert!(does_not_have_roles);
    /// ```
    ///
    /// * `roles` - The roles to check against the manager.
    pub fn not_any(&self, roles: Vec<T>) -> bool {
        self.try_not_any(self.to_role_values(roles))
            .expect("`roles` contain invalid values")
    }

    /// Returns the value of the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRoleUnchecked,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRoleUnchecked, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// impl Into<usize> for MyRole {
    ///     fn into(self) -> usize {
    ///         self as usize
    ///     }
    /// }
    ///
    /// let roles = MyRole::empty();
    ///
    /// // Get value of the manager instance.
    /// let value = roles.get_value();
    ///
    /// assert_eq!(value, 0);
    /// ```
    pub fn get_value(&self) -> usize {
        self.0
    }
}

impl<T> PartialEq<Self> for RoleManagerUnchecked<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for RoleManagerUnchecked<T> {}
