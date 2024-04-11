use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Messages {
    attack: Vec<Message>,
    defend: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    text: String,
    vgs: String,
}

fn main() {
    if let Ok(current_dir) = env::current_dir() {
        eprintln!("Current working directory: {:?}", current_dir);
    } else {
        eprintln!("Failed to get current working directory");
    }
    // Read the JSON file
    let file_path = "/Users/oceon/Github/vgs-me/src-tauri/src/json/messages.json";
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse the JSON content with error handling
    let messages: Messages = match serde_json::from_str(&contents) {
        Ok(messages) => messages,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return;
        }
    };

    // Access the messages
    let attack_messages = &messages.attack;
    let defend_messages = &messages.defend;

    // Do whatever you want with the messages
    println!("Attack messages: {:?}", attack_messages);
    println!("Defend messages: {:?}", defend_messages);
}

