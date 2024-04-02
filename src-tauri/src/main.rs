use serde::Deserialize;
use std::{env, fs, io};

#[derive(Debug, Deserialize)]
struct Message {
    text: String,
    vgs: String,
}

#[derive(Debug, Deserialize)]
struct Messages {
    attack: Vec<Message>,
    defend: Vec<Message>,
    // Add more categories as needed
}

fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Print the current directory for debugging purposes
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the JSON file relative to the current directory
    let json_file_path = current_dir.join("/json").join("messages.json");
    println!("JSON file path: {:?}", json_file_path);

    // Read JSON data from file
    let json_data = fs::read_to_string(&json_file_path)
        .expect("Failed to read JSON file");

    // Deserialize JSON data into Messages struct
    let messages: Messages = serde_json::from_str(&json_data).expect("Failed to deserialize JSON data");

    // Prompt the user to input a VGS code
    println!("Enter VGS code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input to remove whitespace and newlines
    let input = input.trim();

    // Check if the input matches any VGS code in the messages
    let mut found = false;
    for message in &messages.attack {
        if input == message.vgs {
            println!("Correct: {}", message.text);
            found = true;
            break;
        }
    }

    if !found {
        for message in &messages.defend {
            if input == message.vgs {
                println!("Correct: {}", message.text);
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("Incorrect VGS code");
    }
}