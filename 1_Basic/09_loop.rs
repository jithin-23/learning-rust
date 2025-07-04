// Example 1: loop with label and nested loops
fn labeled_loop() {
    let mut count = 0;

    // Outer loop with label 'counting_up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break; // breaks inner loop only
            }

            if count == 2 {
                break 'counting_up; // breaks outer labeled loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

// Example 2: while loop counting down
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Example 3: for loop over an array
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Single main function to run all examples
fn main() {
    println!("--- Labeled loop example ---");
    labeled_loop();

    println!("\n--- While loop example ---");
    while_loop();

    println!("\n--- For loop example ---");
    for_loop();
}
