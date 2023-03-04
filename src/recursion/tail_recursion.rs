#![allow(dead_code)]
fn greatest_common_divisor(m: i32, n: i32) -> i32 {
    if m < n {
        return greatest_common_divisor(n, m);
    }
    let r = m % n;
    if r == 0 {
        n
    } else {
        greatest_common_divisor(n, r)
    }
}

#[cfg(test)]
mod tests {
    use super::greatest_common_divisor;

    #[test]
    fn test_greatest_common_divisor() {
        let result = greatest_common_divisor(10, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_greatest_common_divisor_reverse() {
        let result = greatest_common_divisor(2, 10);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_greatest_common_divisor_not_even() {
        assert_eq!(greatest_common_divisor(9, 2), 1);
    }
}
