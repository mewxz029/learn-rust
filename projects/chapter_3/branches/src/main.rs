fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    is_equal_zero(3);
    is_divisible_by(6);
    if_declare_let();
}

fn is_equal_zero(x: u8) {
    if x != 0 {
        println!("number was something other than zero.");
    }
}

fn is_divisible_by(x: u8) {
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_declare_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
