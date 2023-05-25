use std::io;
mod intro;
use dialoguer::{Select, theme::ColorfulTheme};
fn main() {
    println!("Enter your name:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
    intro::greet(input);
    println!();
    choose();
    
}

fn choose(){
    let options = ["Easy", "Medium", "Hard", "Impossible"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selected option (optional)
        .interact()
        .unwrap();

    println!("Selected: {}", options[selection]);

    match selection {
        0 => println!("You selected Easy"),
        1 => println!("You selected Medium"),
        2 => println!("You selected Hard"),
        3 => println!("You selected Impossible"),
        _ => unreachable!(),
    }
}