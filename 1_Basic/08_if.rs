fn main() {
    let number = 10;

    // Simple if-else condition
    if number > 0 {
        println!("The number is positive");
    } else if number == 0 {
        println!("The number is zero");
    } else {
        println!("The number is negative");
    }

    // Using if as an expression to assign value to a variable
    // Both branches (if and else) must return the same type
    let result = if number % 2 == 0 {
        "even"  // this is a string slice (&str)
    } else {
        "odd"   // this must also be a string slice
    };

    println!("The number is {}", result);

    // Another example: using numbers instead of strings
    let value = if number > 5 {
        100   // integer
    } else {
        200   // must also be integer
    };

    println!("Value is {value}");
}
