use std::io::{self, Write};
mod intro;
mod npc;
use dialoguer::{Select, MultiSelect, theme::ColorfulTheme};
use std::{thread, process};
use std::time::Duration;
fn main() {
    println!("Type `Ctrl` or `Command` C to quit the game at any time.");
    print_typing_effect("Enter your name:".to_string(), 50);

    let mut input = String::new();
    let mut i2 = String::new();
    let selected_option;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    print_typing_effect(format!("Hello, {}!", input.trim()), 50);
    intro::greet(input);
    println!();
    selected_option = choose();
    println!("Type \"bye\" to end the conversation.");
    game(selected_option);
    println!("{}", selected_option);
    choose2();
    println!("Press Enter To Finish The Game");
    io::stdin()
        .read_line(&mut i2)
        .expect("Failed to read line");
    
}

fn choose() -> &'static str {
    let options = ["Easy", "Medium", "Hard", "Impossible"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selected option (optional)
        .interact()
        .unwrap();

    println!("Selected: {}", options[selection]);

    match selection {
        0 => {print_typing_effect("I see, so you're a sissy, too scared to get your hands dirty? That's okay, we'll take it easy on you.".to_string(), 20)},
        1 => {print_typing_effect("I see, so you won't give me up all the way, but you're still too much of coward to choose anything harder than average, kind of lame".to_string(), 20)},
        2 => {print_typing_effect("You think you're a tough guy huh? Why didn't you choose impossible, I guess you're still just a beta".to_string(), 20)},
        3 => {print_typing_effect("I have nothing to say, the choice speaks for itself.".to_string(), 20)},
        _ => unreachable!(),
    }
    options[selection]
}

fn game(con:&str) {
    match con {
        "Easy" => {npc::easy()},
        "Medium" => println!("Meh, its medium"),
        "Hard" => println!("Okay, its getting spicy now"),
        "Impossible" => println!("Now we're talking WHOOOO BABY!"),
        _ => unreachable!(),
    }
}

fn choose2() {
    println!("Press the SPACEBAR to select different options, if you pick the wrong two items together, you die.");
 // Define the options for the multiselect prompt
 let options = &[
    "Banana",
    "Water",
    "Candle",
    "Sodium",
    "Oxygen",
];

// Create the multiselect prompt
let selections = MultiSelect::new()
    .with_prompt("Select multiple options")
    .items(options)
    .interact()
    .unwrap();

// Print the selected options
println!("Selected options:");
for selection in &selections {
    println!("{}", options[*selection]);
}
    // Perform logic based on the selected options
    match selections.as_slice() {
        [1, 3] => {
            let mut i2 = String::new();
            println!("Water and Sodium create a violent Chemical Reaction, which lead to your death");
            println!("Press Enter To Finish The Game");
    io::stdin()
        .read_line(&mut i2)
        .expect("Failed to read line");
            process::exit(0);
            // Perform specific actions for options 1, 2, and 3
        }
        [2, 4] => {
            let mut i2 = String::new();
            println!("The candle's flame reacted with the oxygen to create an explosion, and this lead to your death.");
            println!("Press Enter To Finish The Game");
            io::stdin()
                .read_line(&mut i2)
                .expect("Failed to read line");
                    process::exit(0);
            // Perform specific actions for options 4 and 5
        }
        _ => {
            println!("No specific logic for selected options");
            // Perform default actions if none of the specific cases match
        }
    }

}

fn print_typing_effect(text: String, dur:u64) {
    let mut stdout = io::stdout();

    for c in text.chars() {
        write!(stdout, "{}", c).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(dur)); // Adjust the delay as needed
    }

    println!(); // Move to the next line after the typing effect
}