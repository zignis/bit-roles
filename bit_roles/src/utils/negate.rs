/// Negates the boolean value.
///
/// * `negate` - The value to be negated.
pub(crate) fn negate(value: bool) -> bool {
    !value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_negate_values() {
        assert!(negate(false));
    }
}
