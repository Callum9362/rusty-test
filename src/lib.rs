pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn factorial(number : i128) -> i128{
    let mut factorial : i128 = 1;
    for i in 1..(number+1) { factorial*=i; }
    return factorial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_with_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_mixed_sign_numbers() {
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_divide_valid() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err(String::from("Division by zero")));
    }

    #[test]
    fn test_subtract_positive_numbers() {
        assert_eq!(subtract(3, 2), 1);
    }

    #[test]
    fn test_factorial_positive_numbers_for_0_is_1() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_positive_numbers_for_20() {
        assert_eq!(factorial(20), 2432902008176640000);
    }
}
