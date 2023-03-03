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

#[cfg(test)]
mod tests {
    use super::{fibonacci, linear_recursion, fibonacci_array};

    // #[test]
    fn test_liner_recursion() {
        let n = 5;
        assert_eq!(linear_recursion(n), 120);
    }

    // #[test]
    fn test_fibonance() {
        let n = 10;
        assert_eq!(fibonacci(n), 55);
    }

    // #[test]
    fn test_fibonance_array() {
        let n = 10;
        assert_eq!(fibonacci_array(n), vec![0,1,1,2,3,5,8,13,21,34]);
    }
}
