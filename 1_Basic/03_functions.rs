fn main() {
    greet(); // simple call without arguments

    say_hello("Alice"); // pass string slice

    let sum = add(5, 3); // calling and storing return value
    println!("Sum: {}", sum);

    let doubled = double(10); // calling function with expression return
    println!("Double of 10: {}", doubled);

    let result = multiply_then_add(2, 3, 4);
    println!("(2 * 3) + 4 = {}", result);

    // Function call in a println directly
    println!("Square of 6 is {}", square(6));
}

// --------------------
// ✅ Function Basics
// --------------------

// Function without parameters and no return
fn greet() {
    println!("Hello from Rust!");
}

// Function with a parameter
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

// Function with parameters and return value
fn add(x: i32, y: i32) -> i32 {
    return x + y; // explicit return
}

// Function using implicit return (last expression)
fn double(x: i32) -> i32 {
    x * 2 // no semicolon = implicit return
}

// Function with multiple parameters
fn multiply_then_add(a: i32, b: i32, c: i32) -> i32 {
    (a * b) + c
}

// Direct use of expression return
fn square(x: i32) -> i32 {
    x.pow(2) // x squared
}
