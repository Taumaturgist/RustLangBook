use std::io;

fn main() {
    println!("Hello! This is a Fibonacci generator!");

    loop {
        println!("Enter the number of Fibonacci sequence member (i32 max) to calculate it!");

        let mut fib_number = String::new();
        
        io::stdin()
            .read_line(&mut fib_number)
            .expect("Failed to read line");

        let fib_number: i32 = match fib_number.trim().parse() {
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

fn calculate_fibonacci_pos (num: i32) -> i32 {

    let mut a = vec![0; num as usize];    

    let mut index = 0;

    for element in a {
        if index == 0 {
            element = 0;
            index += 1;
        } else if index == 1 {
            element = 1;
            index += 1;
        } else {
            element = a[index - 2] + a[index - 1];
            index += 1;
        }
    }    

    a[num as usize - 1]
}
