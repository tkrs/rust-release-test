pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn minus(left: i64, right: i64) -> i64 {
    left - right
}

pub fn mul(left: i64, right: i64) -> i64 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn minus_works() {
        let result = minus(2, 3);
        assert_eq!(result, -1);
    }

    #[test]
    fn mul_works() {
        let result = mul(2, 3);
        assert_eq!(result, 6);
    }
}
