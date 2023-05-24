use std::io;
mod intro;
use dialoguer::{Select};
fn main() {
    println!("Enter your name:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
    intro::greet();
    println!();
    choose();
    
}

fn choose(){
    let options = ["Option 1", "Option 2", "Option 3"];

    let selection = Select::new()
        .items(&options)
        .default(0) // Set the default selected option (optional)
        .interact()
        .unwrap();

    println!("Selected: {}", options[selection]);
}