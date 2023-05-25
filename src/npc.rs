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
        ("How are you?", "I'm doing well, thank you!"),
        ("Goodbye", "Goodbye!"),
    ];

    println!("NPC: Welcome! How can I assist you today?");

    loop {
        println!("Player: ");
        let player_input = get_user_input().expect("Failed to read input.");

        if let Some(response) = get_npc_response(&player_input, &npc_responses) {
            println!("NPC: {}", response);
        } else {
            println!("NPC: I'm sorry, I don't understand.");
        }

        if player_input.to_lowercase().contains("bye") {
            break;
        }
    }
}