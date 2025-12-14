use std::io;

fn main() {
    println!("Hello! This is a Fibonacci generator!");

    loop {
        println!("Enter the number of Fibonacci sequence member (i128 max) to calculate it!");

        let mut fib_number = String::new();
        
        io::stdin()
            .read_line(&mut fib_number)
            .expect("Failed to read line");

        let fib_number: i128 = match fib_number.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Error: only digits input allowed");
                continue;
            }    
        };
        
        let result = calculate_fibonacci_pos(fib_number);
        println!("Member of Fibonacci sequence number {fib_number} is {result}!");
        break;
    }    
}

fn calculate_fibonacci_pos (num: i128) -> i128 {

   
}
