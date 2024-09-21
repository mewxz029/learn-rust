fn main() {
    println!("nth: {}", fibonacci(10));
    println!("nth: {}", fibonacci_memoized(10, 0, 1));
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibonacci_memoized(n: u32, a: u32, b: u32) -> u32 {
    if n == 0 {
        return a;
    }
    fibonacci_memoized(n - 1, b, a + b)
}
