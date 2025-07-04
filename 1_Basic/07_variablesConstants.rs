// Constants are always immutable and must have a type.
// They are defined at the top level and can be used anywhere.
const PI: f32 = 3.1415;

fn main() {
    println!("PI constant value: {}", PI);

    // Immutable variable (default behavior in Rust)
    let name = "Alice";
    println!("Name: {}", name);

    // name = "Bob"; // This would cause an error because `name` is immutable

    // Mutable variable using `mut` keyword
    let mut age = 25;
    println!("Age: {}", age);

    age = 26; // This is allowed because `age` is mutable
    println!("Updated age: {}", age);

    // Shadowing: re-declaring a variable with the same name
    let age = age + 1; // This creates a new variable `age` that shadows the old one
    println!("Shadowed age: {}", age);

    // Inner scope
    {
        let age = age * 2; // New `age` only in this block
        println!("Age inside block: {}", age);
    }

    // Outside the block, the previous `age` still holds
    println!("Age outside block: {}", age);

    // Scope example
    let outer = "I am outside";

    {
        let inner = "I am inside";
        println!("{}", inner);  // Can access inner
        println!("{}", outer);  // Can access outer
    }

    // println!("{}", inner); // This would cause an error: `inner` is not in scope here
    println!("{}", outer); // Still accessible
}
