fn main() {
    println!("=== 🧠 Ownership and Immutable Borrowing ===");
    let name = String::from("Jithin");

    let r1 = &name; // Immutable borrow
    let r2 = &name; // Another immutable borrow
    println!("Hello, {} and {}!", r1, r2); // ✅ Multiple immutable borrows are allowed

    // let r3 = &mut name; // ❌ ERROR: Cannot borrow as mutable when immutable references exist

    // -----------------------------------------------------
    println!("\n=== ✍️ Mutable Borrowing After Immutable References Expire ===");
    let mut name2 = String::from("Jithin");

    {
        let r1 = &name2;
        println!("Read: {}", r1); // Immutable borrow inside inner scope
    } // r1 is dropped here

    let r2 = &mut name2;
    r2.push_str(" Joseph");
    println!("Modified: {}", r2); // ✅ Now mutable borrow is allowed

    // -----------------------------------------------------
    println!("\n=== 🧰 Function Arguments and Borrowing ===");
    let mut username = String::from("Jithin");

    greet(&username);           // Immutable borrow passed to function
    add_suffix(&mut username);  // Mutable borrow passed to function
    println!("After modification: {}", username); // "Jithin the Rustacean"

    // -----------------------------------------------------
    println!("\n=== ⚠️ Invalid Mixed Borrowing ===");
    let mut x = String::from("oops");

    let r1 = &x;
    println!("Immutable: {}", r1); // Using r1

    // Uncommenting the next line would cause an error
    // let r2 = &mut x; // ❌ ERROR: cannot borrow as mutable when immutable borrow exists
    // println!("{}, {}", r1, r2);
}

fn greet(name: &String) {
    println!("Hello, {}!", name); // read-only access
}

fn add_suffix(name: &mut String) {
    name.push_str(" the Rustacean"); // mutable access
}
