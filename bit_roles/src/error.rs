use thiserror::Error;

/// The error raised when working with role values.
#[derive(Error, Debug)]
pub enum RoleError {
    /// Raised when the provided role holds a value that is neither zero nor a
    /// power of two.
    #[error("invalid role value: `{0}` is neither zero nor a power of two")]
    InvalidRole(usize),
}
