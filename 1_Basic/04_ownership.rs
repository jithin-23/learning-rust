
//1- Each value in Rust has a variable that is its owner

/*fn main () {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of `{}` is {}", s1,len);
}*/

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

//2- There can only be one owner at a time

/*
fn main() {
    let s1 = String::from("RUST");
    let s2 = s1;

    println!("{}",s1); //This shows error because s1 is no longer the owner
    println!("{}",s2);
}*/

// 3- When owner goes out of scope, value will be dropped

fn main() {
    let s1 = String::from("RUST");
    let _len = calculate_length(&s1);
    println!("Length of '{}' is {} ",s1,_len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn printLost(){
    println!("{}", &s1);
}

