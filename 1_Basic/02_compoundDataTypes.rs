fn main() {
    // ------------------
    // TUPLES
    // ------------------
    let person: (i32, f64, char) = (25, 72.5, 'M');
    println!("Person: {:?}", person); // prints the whole tuple
    println!("Age: {}, Weight: {}, Gender: {}", person.0, person.1, person.2);

    // ------------------
    // ARRAYS
    // ------------------
    let numbers: [i32; 4] = [10, 20, 30, 40];
    println!("Numbers: {:?}", numbers); // prints whole array

    let zeros = [0; 5]; // initializes all elements to 0
    println!("Zeros: {:?}", zeros);

    // ------------------
    // SLICES
    // ------------------
    let slice = &numbers[1..3]; // from index 1 to 2
    println!("Slice of numbers[1..3]: {:?}", slice);

    print_slice(&numbers[..]); // passing whole array as slice

    // ------------------
    // STRING
    // ------------------
    // String is a heap-allocated, growable string
    let mut s: String = String::from("Hello");
    s.push_str(", Rust!"); // appending to String
    println!("String: {}", s);

    // ------------------
    // STRING SLICE (&str)
    // ------------------
    // String slices are immutable views into a string (not owned)
    let greeting: &str = "Hi there!";
    println!("String slice: {}", greeting);

    // You can also take a slice from a String
    let part = &s[0..5]; // slice "Hello" from "Hello, Rust!"
    println!("Slice of String: {}", part);
}

fn print_slice(slice: &[i32]) {
    println!("Printing from slice:");
    for value in slice {
        println!("{}", value);
    }
}
