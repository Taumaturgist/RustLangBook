fn main() {
    println!("Fun with functions!");

    another_function();

    another_function_with_arg(5);

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(x);

    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(y: i32) -> i32 {
    y + 1
}