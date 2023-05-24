use std::io;
mod intro;
fn main() {
    println!("Enter your name:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
    intro::greet();
}

