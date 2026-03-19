/// A simple calculator module for demo purposes.
pub mod calculator {
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    pub fn subtract(a: i64, b: i64) -> i64 {
        a - b
    }

    pub fn multiply(a: i64, b: i64) -> i64 {
        a * b
    }

    pub fn divide(a: i64, b: i64) -> Result<i64, &'static str> {
        if b == 0 {
            return Err("division by zero");
        }
        Ok(a / b)
    }

    pub fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a: u64 = 0;
                let mut b: u64 = 1;
                for _ in 2..=n {
                    let temp = b;
                    b += a;
                    a = temp;
                }
                b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calculator::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(7, 2), Ok(3));
        assert_eq!(divide(5, 0), Err("division by zero"));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }
}
