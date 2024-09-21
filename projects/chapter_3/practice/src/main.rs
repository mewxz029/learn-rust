fn main() {
    println!("nth: {}", fibonacci(10));
    println!("nth: {}", fibonacci_memoized(10, 0, 1));

    convert_temperature(32.0, 'f');
    convert_temperature(32.0, 'c');
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

fn convert_temperature(x: f64, target_unit: char) {
    if target_unit == 'c' {
        println!(
            "fahrenheit to degree celsius: {:.2}",
            ((x - 32.0) * 5.0 / 9.0)
        );
    } else {
        println!(
            "degree celsius to fahrenheit: {:.2}",
            ((x * 9.0 / 5.0) + 32.0)
        );
    }
}
