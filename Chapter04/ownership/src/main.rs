fn main() {
    println!("Fun with Ownership");

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);   // s's value moves into the function and so is no longer valid here
    
    //println!("{s}"); //this is not working (as planned), compilation error

    let s1 = gives_ownership();        // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    println!("{s1}, {s3}");

    let s4 = String::from("hello");

    // a referense arg to avoid movement of value
    let len = calculate_length(&s4);

    println!("The length of '{s4}' is {len}.");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn gives_ownership() -> String {       // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and moves out to the calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

// a referense arg to avoid movement of value
fn calculate_length(s: &String) -> usize {
    s.len()
}