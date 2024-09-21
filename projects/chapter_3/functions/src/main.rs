fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        // INFO: with semicolon at then end we call 'statement' (perform action not return value)
        let x = 3;
        // INFO: without semicolon at then end we call 'expression' (evaluate to return value)
        // mostly use at the end of function
        x + 1
    };
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
