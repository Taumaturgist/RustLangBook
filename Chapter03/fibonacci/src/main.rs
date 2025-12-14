use std::io;

fn main() {
    println!("Hello! This is a Fibonacci generator!");   

    loop {
        println!("Enter the index of Fibonacci sequence member to calculate it!\nIndex must be in [-184; 184] interval because of i128 type used.");
        
        let mut fib_index = String::new();
        let mut quit_string = String::new();

        io::stdin()
            .read_line(&mut fib_index)
            .expect("Failed to read line");

        let fib_index: i16 = match fib_index.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Error: only digits input allowed");
                continue;
            }    
        };

        if fib_index > 184 || fib_index < -184 {
            println!("Error: index must be in [-184; 184] interval because of i128 type used");
            continue;
        }
        
        let result = calculate_fibonacci_pos(fib_index);
        println!("Member of Fibonacci sequence at index {fib_index} is {result}!");

        println!("One more calculation? type 'y' to repeat, type anything else to quit");
        
        io::stdin()
            .read_line(&mut quit_string)
            .expect("Failed to read line");

        let quit_string = quit_string.trim();

        if quit_string == "y" {            
            continue;
        } else {
            println!("Good bye!");
        }

        break;
    }    
}

fn calculate_fibonacci_pos (num: i16) -> i128 {

   let index = num;
   let mut counter: i16;
   let mut result: i128 = 0;
   let mut fib_minus_one: i128;
   let mut fib_minus_two: i128 = 0;

   if index == 1 {
       result = 1;
   } else if index == -1 {
       result = -1;
   } else if index > 1 {
        fib_minus_one = 1;
        counter = 1;
        while counter < index {
            result = fib_minus_two + fib_minus_one;
            fib_minus_two = fib_minus_one;
            fib_minus_one = result;
            counter += 1;
        }
   } else if index < -1 {
        counter = -1;
        fib_minus_one = -1;
        while counter > index {
            result = fib_minus_two + fib_minus_one;
            fib_minus_two = fib_minus_one;
            fib_minus_one = result;
            counter -= 1;
        }
   }

   result
}
