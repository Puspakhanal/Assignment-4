#[cfg(test)]
mod tests {
    use super::super::calculator::calculate;
    #[test]
    fn test_addition() {
        assert_eq!(calculate("add", 2.0, 2.0).unwrap(), 4.0);
        assert_eq!(calculate("+", 2.0, 2.0).unwrap(), 4.0);
    }
    #[test]
    fn test_subtraction() {
        assert_eq!(calculate("sub", 3.0, 2.0).unwrap(), 1.0);
        assert_eq!(calculate("-", 3.0, 2.0).unwrap(), 1.0);
    }
    #[test]
    fn test_multiplication() {
        assert_eq!(calculate("mul", 3.0, 2.0).unwrap(), 6.0);
        assert_eq!(calculate("*", 3.0, 2.0).unwrap(), 6.0);
    }
    #[test]
    fn test_division() {
        assert_eq!(calculate("div", 4.0, 2.0).unwrap(), 2.0);
        assert_eq!(calculate("/", 4.0, 2.0).unwrap(), 2.0);
    }
    #[test]
    fn test_division_by_zero() {
        assert!(calculate("div", 4.0, 0.0).is_err());
        assert!(calculate("/", 4.0, 0.0).is_err());
    }
    #[test]
    fn test_invalid_operation() {
        assert!(calculate("mod", 4.0, 2.0).is_err());
    }
}
