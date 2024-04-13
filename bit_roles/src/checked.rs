use crate::RoleVariant;
use std::{
    marker::PhantomData,
    ops::{
        BitAnd,
        BitAndAssign,
        BitOrAssign,
    },
};

/// Bit role trait with compile-time value checks. Implements [RoleManager] for
/// a role enum.
pub trait BitRoleImpl<T> {
    /// Creates a new [RoleManager] instance with the default value.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// // Create an empty manager instance.
    /// let mut roles = MyRole::empty();
    ///
    /// assert_eq!(roles.get_value(), 0);
    /// ```
    fn empty() -> RoleManager<T>;
    /// Creates a new [RoleManager] instance with the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// // Create a manager instance with the provided value.
    /// let mut roles = MyRole::from_value(5);
    ///
    /// assert_eq!(roles.get_value(), 5);
    /// ```
    ///
    /// * `value` - The value for the manager.
    fn from_value(value: usize) -> RoleManager<T>;
}

/// The default role manager with compile-time value checks.
#[derive(Debug)]
pub struct RoleManager<T>(pub usize, pub PhantomData<T>);

impl<T> RoleManager<T>
where
    T: RoleVariant,
{
    /// Adds a single role to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
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
        self.0.bitor_assign(role.into());
        self
    }

    /// Adds multiple roles to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        roles.into_iter().for_each(|role| {
            self.add_one(role);
        });

        self
    }

    /// Removes a single role from the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
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
        self.0.bitand_assign(!role.into());
        self
    }

    /// Removes multiple roles from the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        roles.into_iter().for_each(|role| {
            self.remove_one(role);
        });

        self
    }

    /// Checks whether a single role is assigned to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
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
        self.0.bitand(role.into()) != 0
    }

    /// Checks whether each of the roles is assigned to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        roles
            .into_iter()
            .all(|role| self.0.bitand(Into::<usize>::into(role)) != 0)
    }

    /// Checks whether any one of the roles is assigned to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        roles
            .into_iter()
            .any(|role| self.0.bitand(Into::<usize>::into(role)) != 0)
    }

    /// Checks whether a single role is not assigned to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
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
        !self.has_one(role)
    }

    /// Checks whether each of the roles is not assigned to the manager
    /// instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        !self.has_all(roles)
    }

    /// Checks whether any of the roles is not assigned to the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    ///     Member = 2,
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
        !self.has_any(roles)
    }

    /// Returns the value of the manager instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::BitRole;
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
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

impl<T> PartialEq<Self> for RoleManager<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for RoleManager<T> {}
