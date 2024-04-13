use crate::{
    utils::is_valid_role,
    RoleError,
    RoleVariant,
};

/// The enum holding value of a role.
#[derive(Debug, Copy, Clone)]
pub enum RoleValue<T>
where
    T: RoleVariant,
{
    /// Variant that can accept role enum variants.
    Role(T),
    /// Variant that can accept literal integer values.
    Raw(usize),
}

impl<T> RoleValue<T>
where
    T: RoleVariant,
{
    /// Creates a new [RoleValue] instance from a role without performing the
    /// validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRole,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// let value = RoleValue::from_role(MyRole::Staff);
    ///
    /// assert_eq!(value, RoleValue::Role(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role variant.
    pub fn from_role(role: T) -> Self {
        RoleValue::Role(role)
    }

    /// Validates a role and creates a new [RoleValue] instance from a it.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRole,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// let value = RoleValue::try_from_role(MyRole::Staff).expect("invalid role");
    ///
    /// assert_eq!(value, RoleValue::Role(MyRole::Staff));
    /// ```
    ///
    /// * `role` - The role variant.
    pub fn try_from_role(role: T) -> Result<Self, RoleError> {
        is_valid_role(role.into())
            .then_some(RoleValue::Role(role))
            .ok_or(RoleError::InvalidRole(role.into()))
    }

    /// Creates a new [RoleValue] instance from an integer value without
    /// performing the validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRole,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// let value: RoleValue<MyRole> = RoleValue::from_usize(4);
    ///
    /// assert_eq!(value, RoleValue::Raw(4));
    /// ```
    ///
    /// * `value` - The magnitude.
    pub fn from_usize(value: usize) -> Self {
        RoleValue::Raw(value)
    }

    /// Validates an integer value and creates a new [RoleValue] instance from
    /// it.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_roles::{
    ///     BitRole,
    ///     RoleValue,
    /// };
    ///
    /// #[derive(Debug, BitRole, Copy, Clone)]
    /// enum MyRole {
    ///     None = 0,
    ///     Staff = 1,
    /// }
    ///
    /// let value: RoleValue<MyRole> = RoleValue::try_from_usize(4).expect("invalid value");
    ///
    /// assert_eq!(value, RoleValue::Raw(4));
    /// ```
    ///
    /// * `value` - The magnitude.
    pub fn try_from_usize(value: usize) -> Result<Self, RoleError> {
        is_valid_role(value)
            .then_some(RoleValue::Raw(value))
            .ok_or(RoleError::InvalidRole(value))
    }
}

impl<T> From<RoleValue<T>> for usize
where
    T: RoleVariant,
{
    fn from(val: RoleValue<T>) -> Self {
        match val {
            RoleValue::Role(role) => role.into(),
            RoleValue::Raw(value) => value,
        }
    }
}

impl<T> PartialEq<Self> for RoleValue<T>
where
    T: RoleVariant,
{
    fn eq(&self, other: &Self) -> bool {
        Into::<usize>::into(*self) == Into::<usize>::into(*other)
    }
}

impl<T> Eq for RoleValue<T> where T: RoleVariant {}
