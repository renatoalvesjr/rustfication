#[cfg(test)]
mod op_tests {
    use crate::operations::*;

    #[test]
    fn test_addition() {
        assert_eq!(addition::add(2, 3), 5);
        assert_eq!(addition::add(-1, 1), 0);
        assert_eq!(addition::add(0, 0), 0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction::minus(5, 3), 2);
        assert_eq!(subtraction::minus(3, 5), -2);
        assert_eq!(subtraction::minus(0, 0), 0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(multiplication::multiply(2, 3), 6.0);
        assert_eq!(multiplication::multiply(-1, 1), -1.0);
        assert_eq!(multiplication::multiply(0, 5), 0.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(division::divide(6, 3), 2.0);
        assert_eq!(division::divide(5, 2), 2.5); // Assuming integer division
        assert_eq!(division::divide(0, 1), 0.0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_division_by_zero() {
        division::divide(1, 0);
    }
}