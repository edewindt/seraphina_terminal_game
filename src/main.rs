use std::io;
mod intro;
use dialoguer::{Select, MultiSelect, theme::ColorfulTheme};
fn main() {
    println!("Enter your name:");

    let mut input = String::new();
    let mut i2 = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
    intro::greet(input);
    println!();
    choose();
    println!();
    choose2();
    println!("Press Enter To Finish The Game");
    io::stdin()
        .read_line(&mut i2)
        .expect("Failed to read line");
    
}

fn choose() {
    let options = ["Easy", "Medium", "Hard", "Impossible"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selected option (optional)
        .interact()
        .unwrap();

    println!("Selected: {}", options[selection]);

    match selection {
        0 => println!("I see, so you're a sissy, too scared to get your hands dirty? That's okay, we'll take it easy on you."),
        1 => println!("I see, so you won't give me up all the way, but you're still too much of coward to choose anything harder than average, kind of lame"),
        2 => println!("You think you're a tough guy huh? Why didn't you choose impossible, I guess you're still just a beta"),
        3 => println!("I have nothing to say, the choice speaks for itself."),
        _ => unreachable!(),
    }
}

fn choose2() {
    println!("Press the SPACEBAR to select different options");
 // Define the options for the multiselect prompt
 let options = &[
    "Option 1",
    "Option 2",
    "Option 3",
    "Option 4",
    "Option 5",
];

// Create the multiselect prompt
let selections = MultiSelect::new()
    .with_prompt("Select multiple options")
    .items(options)
    .interact()
    .unwrap();

// Print the selected options
println!("Selected options:");
for selection in selections {
    println!("{}", options[selection]);
}
}