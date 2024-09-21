fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");

    // NOTE: Scalar Types has 4 primary types:
    // integers, floating-point numbers, boolean and characters

    // NOTE: Integer Types
    // length  | signed | unsigned
    // 8-bit   | i8     |  u8
    // 16-bit  | i16    |  u16
    // 32-bit  | i32    |  u32
    // 64-bit  | i64    |  u64
    // 128-bit | i128   |  u128
    // arch    | isize  |  usize

    // NOTE: signed & unsigned
    // signed => -2^n-1 to 2^n-1 - 1
    // unsigned => 0 to 2^n - 1
    // ex: i8 => -(2^(8-1)) to 2^(8-1)-1 => -128 to 127
    // ex: u8 => 0 to 2^8 - 1 => 0 to 255

    // NOTE: integer overflow
    // if value out of range in debug mode program will panic
    // if in release mode --release program doesn't check overflow and not panic but give
    // unexpected value ex: u8 is 0-256 and the value 257 become 1
    // it also can handle with wrapping_*, checked_*, overflowing_* and saturating_*

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}, y: {y}");

    // NOTE: Floating-Point Types
    // Rust's floating-point types are f32 and f64 which are 32 bits and 64 bits
    // default type is f64
    // all floating-point are signed (negative to positive value)
    // according to the IEEE-754 standard

    // NOTE: Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // NOTE: The Boolean Type
    // just 1 byte
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {t}, f: {f}");

    // NOTE: The Character Type
    // Note that we specify char literals with single quotes.
    // char type is 4 bytes in size and represents a Unicode Scalar Value.
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // NOTE: Compound Types
    // can group multiple values into one type.
    // has two primitive compound type: tuples and arrays.

    // NOTE: The Tuple Type
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Tuple w/o any values call 'unit' represent an empty value.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    println!("five_hundred: {five_hundred}");

    // NOTE: The Array Type
    // array have a fixed length and same type.
    let a = [1, 2, 3, 4, 5];
    println!("a.0: {0}", a[0]);

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[1]: {0}", a[1]);

    let _a = [3; 5];
    // NOTE: same as: let a = [3, 3, 3, 3, 3];
}
