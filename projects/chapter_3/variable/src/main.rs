fn main() {
    // NOTE: this variable is immutable by default
    // let x = 5;
    // NOTE: add `mut` to make it mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // NOTE: constant declaring must have type annotation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is: {THREE_HOURS_IN_SECONDS}");

    // NOTE: Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // NOTE: if use let mut spaces = "    ";
    // then spaces = spaces.len();
    // will got error type
}
