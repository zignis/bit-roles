/// Validates a role value.
///
/// * `value` - The value of role.
pub fn is_valid_role(value: usize) -> bool {
    value == 0 || value.is_power_of_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_roles() {
        assert!(is_valid_role(0));
        assert!(is_valid_role(1));
        assert!(is_valid_role(2));
        assert!(is_valid_role(4));
    }

    #[test]
    fn can_invalidate_roles() {
        assert!(!is_valid_role(3));
        assert!(!is_valid_role(5));
    }
}
