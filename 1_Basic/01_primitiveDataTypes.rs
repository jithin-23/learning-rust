fn main() {
    // Integer types
    let a: i32 = -10;       // signed 32-bit integer
    let b: u32 = 20;        // unsigned 32-bit integer
    let c: i64 = 123456789; // signed 64-bit integer

    // Floating-point types
    let x: f32 = 3.14;      // 32-bit float
    let y: f64 = 2.7182818; // 64-bit float (default)

    // Boolean
    let is_rust_fun: bool = true;

    // Character
    let letter: char = 'R';         // single Unicode scalar value
    let emoji: char = '🚀';         // emoji is also a char in Rust

    // Printing all values
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("x: {}, y: {}", x, y);
    println!("is_rust_fun: {}", is_rust_fun);
    println!("letter: {}, emoji: {}", letter, emoji);
}
