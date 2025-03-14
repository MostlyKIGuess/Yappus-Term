use std::io::{self, Write};
use std::env;
use crate::memory::{ save_chat, ChatLog};

mod memory;
mod api;

fn main() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY").expect("API key missing");
    let history_path = "chat_history.json";

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // usage is kinda like yappus "query goes here"
        let query = args[1..].join(" ");
        process_query(&query, &api_key, history_path);
    } else {
        interactive_mode(&api_key, history_path);
    }
}

fn process_query(query: &str, api_key: &str, history_path: &str) {
    match api::get_gemini_response(query, api_key) {
        Ok(response) => {
            println!("{}", response);
            
            let log_entry = ChatLog {
                user: query.to_string(),
                bot: response.clone(),
            };
            save_chat(&log_entry, history_path);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn interactive_mode(api_key: &str, history_path: &str) {
    let chat_history = memory::load_chat(history_path);
    
    // kinda for debugging onli
    for entry in &chat_history {
        println!("You: {}\nGemini: {}\n", entry.user, entry.bot);
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        process_query(input, api_key, history_path);
    }
}
