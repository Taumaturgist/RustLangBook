use std::io;

fn main() {
    println!("Hello, this is a Fahrenheit to Celsius and vice versa converter!");

    loop {
        println!("Please, type 'f' for Fahrenheit or 'c' for Celsius input selection, type 'q' for exit");

        let mut scale = String::new();
        

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale = scale.trim();

        if scale == "f" {
            println!("Fahrenheit input confirmed!");
            convert_fahr2cels();
        } else if scale == "c" {
            println!("Celsius input confirmed!");
            convert_cels2fahr();
        }  else if scale == "q" {
            println!("Good bye!");
            break;
        } else {
            println!("Error: no such input!");
        }
    }    
}

fn convert_fahr2cels() {
    loop {
        println!("Please, enter your Fahrenheit degree");

        let mut f_degree = String::new();

        io::stdin()
            .read_line(&mut f_degree)
            .expect("Failed to read line");
        
        let f_degree: f32 = match f_degree.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Error: only digits input allowed");
                continue;
            }            
        };

        if f_degree < -459.67 {
            println!("Your degree is below Absolute Zero, convertion impossible!");
            continue;
        }

        let c_degree_result = (f_degree - 32.0) / 1.8;
        
        println!("Convertion success: {f_degree} Fahrenheit equals {c_degree_result} Celsius!");

        break;
    }
}

fn convert_cels2fahr() {
    loop {
        println!("Please, enter your Celsius degree");

        let mut c_degree = String::new();

        io::stdin()
            .read_line(&mut c_degree)
            .expect("Failed to read line");
        
        let c_degree: f32 = match c_degree.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Error: only digits input allowed");
                continue;
            }            
        };

        if c_degree < -273.15 {
            println!("Your degree is below Absolute Zero, convertion impossible!");
            continue;
        }

        let f_degree_result = c_degree * 1.8 + 32.0;
        
        println!("Convertion success: {c_degree} Celcius equals {f_degree_result} Fahrenheit!");

        break;
    }
}
