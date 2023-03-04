#![allow(dead_code)]
/// The linear recursion is function to call itself one time.
/// This is function to calculator factorial
fn linear_recursion(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * linear_recursion(n - 1)
}

/// Function to implement the fibonacci sequence
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Again function to calculate fibonacci but return back a array fibonacci
/// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
fn fibonacci_array(n: usize) -> Vec<i32> {
    let mut fibo: Vec<i32> = Vec::with_capacity(n);

    for i in 0..n {
        match i {
            0 => fibo.push(0),
            1 => fibo.push(1),
            _ => {
                let fib_iter = fibo[i - 1] + fibo[i - 2];
                fibo.push(fib_iter)
            }
        }
    }
    fibo
}

/// Use recursion to calculate Sum = 1 + 2 + 3 + .. + n
fn cal_sum(n : i32) -> i32 {
    if n > 1 {
        return n + cal_sum(n - 1);
    }
    n
}

// Use Recursion to calculate Sum of a number S = 12182 = 1 + 2 + 1 + 8 + 2
// Use Recursion to calculate Ackerman
//   Acker(m, n)
//   n + 1 if m = 0
//   acker(m-1, 1) if n = 0;
//   acker(m-1, acker(m, n - 1))
// Use recursion to calculate S(n) = 1^2 + 2^2 + 3^2 + .. + n^2 (n > 0)
// Use recursion to reverse the number S = 1234 => 4321

#[cfg(test)]
mod tests {
    use super::{fibonacci, fibonacci_array, linear_recursion, cal_sum};

    // #[test]
    fn test_linear_recursion() {
        let n = 5;
        assert_eq!(linear_recursion(n), 120);
    }

    // #[test]
    fn test_fibonance() {
        let n = 10;
        assert_eq!(fibonacci(n), 55);
    }

    #[test]
    fn test_cal_sum ()  {
        let n = 15;
        assert_eq!(cal_sum(n), 120);
    }

    // #[test]
    fn test_fibonance_array() {
        let n = 10;
        assert_eq!(fibonacci_array(n), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
