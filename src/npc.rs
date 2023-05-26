use std::io::{self, BufRead};

fn get_user_input() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_npc_response<'a>(player_input: &str, npc_responses: &'a Vec<(&str, &str)>) -> Option<&'a str> {
    for &(trigger, response) in npc_responses {
        if player_input.to_lowercase().contains(&trigger.to_lowercase()) {
            return Some(response);
        }
    }
    None
}

pub fn easy() {
    let npc_responses: Vec<(&str, &str)> = vec![
        ("Hello", "Hi there!"),
        ("Help", "How can I help you today?"),
        ("How are you?", "I'm doing well, thank you!"),
        ("Goodbye", "Goodbye!"),
    ];
    println!("This NPC only responds to: \"Hello\", \"How are you?\", \"Help\", and \"Goodbye\"");
    println!("NPC: Welcome! How can I assist you today?");

    loop {
        println!("Player: ");
        let player_input = get_user_input().expect("Failed to read input.");

        if let Some(response) = get_npc_response(&player_input, &npc_responses) {
            println!("NPC: {}", response);
        } else {
            println!("NPC: I'm sorry, I don't understand.");
        }

        if player_input.to_lowercase().contains("help") {
            help_options();
        }

        if player_input.to_lowercase().contains("bye") {
            break;
        }
    }
}


fn help_options() -> &'static str {
    let options = ["Rules", "Inventory", "My Backstory"];

    let selection = crate::Select::with_theme(&crate::ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selected option (optional)
        .interact()
        .unwrap();

    println!("Selected: {}", options[selection]);

    match selection {
        0 => {crate::print_typing_effect("There are no rules silly".to_string(), 20)},
        1 => {crate::print_typing_effect("You currently have 0 items".to_string(), 20)},
        2 => {crate::print_typing_effect("I was trapped in a terminal interface, and could never escape, make sure you never face the same fate.".to_string(), 20)},
        _ => unreachable!(),
    }
    println!("Type \"help\" again if you need help with anything else.");
    options[selection]
}