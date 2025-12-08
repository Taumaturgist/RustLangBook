fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    println!("The value of y on first declare is: {y}");

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("How many spaces? {spaces}");

    // addition
    let sum = 5 + 10;
    println!("sum 5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("diff 95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient 56.7 / 32.2 = {quotient}");
    
    let truncated = -5 / 3; // Results in -1
    println!("truncated -5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder 43 % 5 = {remainder}");

    let tup = (500, 6.4, 1);
    //println!("The tuple is: {tup}");

    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
