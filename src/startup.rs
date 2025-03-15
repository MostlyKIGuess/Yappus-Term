use std::io::{self, Write};
use std::path::PathBuf;
use std::fs;
use std::cmp::min;

pub fn display_version() {
    println!("\n=== Yappus Terminal v0.1.0 ===");
    println!("A terminal interface for Google Gemini AI");
    println!("Author: MostlyK <bruvistrue93@gmail.com>");
    println!("Repository: https://github.com/mostlyk/yappus-term");
    println!("License: MIT");
}

pub fn check_api_key(config_dir: &PathBuf) {
    let api_key_file = config_dir.join("api_key");
    
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        println!("API key is set in environment variables.");
        println!("API key: {}...", &key[0..min(5, key.len())]);
    } else if let Ok(key) = std::fs::read_to_string(&api_key_file) {
        if !key.trim().is_empty() {
            println!("API key is saved in config file.");
            println!("API key: {}...", &key[0..min(5, key.len())]);
        } else {
            println!("API key is not set.");
        }
    } else {
        println!("API key is not set.");
    }
}

pub fn reset_api_key(config_dir: &PathBuf) {
    let api_key_file = config_dir.join("api_key");
    
    println!("Resetting API key...");
    
    
    if api_key_file.exists() {
        if let Err(e) = fs::remove_file(&api_key_file) {
            eprintln!("Error removing API key file: {}", e);
            return;
        }
    }
    
    println!("Please enter your new Gemini API key:");
    println!("(You can get one from https://ai.google.dev/)");
    println!("Visit https://aistudio.google.com/app/apikey to create an API key");
    
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read API key");
    let key = key.trim().to_string();
    
    if key.is_empty() {
        println!("No API key provided. Operation cancelled.");
        return;
    }
    
    match std::fs::write(api_key_file, &key) {
        Ok(_) => println!("API key reset successfully!"),
        Err(e) => eprintln!("Failed to save API key: {}", e)
    }
}

pub fn get_api_key(config_dir: &PathBuf) -> String {
    let api_key_file = config_dir.join("api_key");
    
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        return key;
    }
    
    // if api already exits
    if let Ok(key) = std::fs::read_to_string(&api_key_file) {
        if !key.trim().is_empty() {
            return key.trim().to_string();
        }
    }
    
    println!("Gemini API key not found. Please enter your API key:");
    println!("(You can get one from https://ai.google.dev/)");
    println!("Visit https://aistudio.google.com/app/apikey to create an API key");
    
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read API key");
    let key = key.trim().to_string();
    
    std::fs::write(api_key_file, &key).expect("Failed to save API key");
    println!("API key saved successfully!");
    
    key
}

// first timeers
pub fn initial_setup(config_dir: &PathBuf) -> bool {
    println!("\n=== Welcome to Yappus Terminal ===");
    println!("A Gemini AI-powered terminal assistant\n");
    
    if let Err(e) = std::fs::create_dir_all(config_dir) {
        eprintln!("Failed to create config directory: {}", e);
        return false;
    }
    
    let config_file = config_dir.join("config.json");
    if config_file.exists() && !is_forced_setup() {
        return true; 
    }
    
    println!("This appears to be your first time running Yappus.");
    println!("Let's set up your configuration.");
    
    let _api_key = get_api_key(config_dir);
    
    println!("\nChoose your preferred Gemini model:");
    println!("1. GEMINI_1_5_FLASH (default, fast responses)");
    println!("2. GEMINI_1_5_PRO (more capable)");
    println!("3. GEMINI_1_5_PRO_002 (latest pro version)");
    println!("4. GEMINI_1_5_FLASH_002 (newer flash model)");
    println!("5. GEMINI_1_5_FLASH_8B (smaller, faster model)");
    println!("6. GEMINI_1_0_PRO (original model)");
    print!("Enter your choice [1-6] or press Enter for default: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let choice = choice.trim();
    
    let model = match choice {
        "1" | "" => "GEMINI_1_5_FLASH",
        "2" => "GEMINI_1_5_PRO",
        "3" => "GEMINI_1_5_PRO_002",
        "4" => "GEMINI_1_5_FLASH_002",
        "5" => "GEMINI_1_5_FLASH_8B",
        "6" => "GEMINI_1_0_PRO",
        _ => "GEMINI_1_5_FLASH" // def
    };
    
    let config = serde_json::json!({
        "model": model
    });
    
    match std::fs::write(&config_file, serde_json::to_string_pretty(&config).unwrap()) {
        Ok(_) => {
            println!("\nConfiguration complete! Using model: {}", model);
            println!("\nTip: You can change models later with the 'yappus model MODEL_NAME' command");
            println!("Tip: Use 'yappus --help' to see all available commands");
            true
        },
        Err(e) => {
            eprintln!("Failed to save configuration: {}", e);
            false
        }
    }
}

fn is_forced_setup() -> bool {
    std::env::args().any(|arg| arg == "setup" || arg == "--setup")
}

pub fn display_help() {
    println!("\n=== Yappus Terminal Help ===");
    println!("Command Line Usage:");
    println!("  yappus \"Your question here\"    - Ask a question directly");
    println!("  yappus model [MODEL_NAME]    - View or change the Gemini model");
    println!("  yappus history              - View your chat history");
    println!("  yappus clear-history        - Clear your chat history");
    println!("  yappus file PATH [QUERY]    - Include file content in your query");
    println!("  yappus setup                - Run the setup process again");
    println!("  yappus version              - Show version information");
    println!("  yappus config               - Display current configuration");
    println!("  yappus key [--reset]        - Check or reset API key");
    println!("  yappus export [PATH]        - Export chat history to file");
    
    println!("\nInteractive Mode Commands:");
    println!("  /help         - Show this help message");
    println!("  /model [name] - View or change the Gemini model");
    println!("  /history      - View your chat history");
    println!("  /ls [path]      - List files in current or specified directory");
    println!("  /file <path> [q]- Include file content in your query");
    println!("  /clearhistory - Clear your chat history");
    println!("  /setup        - Run the setup process again");
    println!("  /version      - Show version information");
    println!("  /config       - Display current configuration");
    println!("  /key [reset]  - Check or reset API key");
    println!("  /export [path]- Export chat history to file");
    println!("  /clear        - Clear the terminal screen");
    println!("  exit          - Exit the application");
    
    println!("\nAvailable models:");
    println!("  - GEMINI_1_5_FLASH (default - good balance)");
    println!("  - GEMINI_1_5_PRO_002 (most powerful)");
    println!("  - GEMINI_1_5_PRO (very capable)");
    println!("  - GEMINI_1_5_FLASH_002 (good performance)");
    println!("  - GEMINI_1_5_FLASH_8B (fastest responses)");
    println!("  - GEMINI_1_0_PRO (original model)");
    
    println!("\nExamples:");
    println!("  > How do I find large files in Linux?");
    println!("  > Write a bash script to backup my home directory");
    println!("  > What's the difference between tar and zip?");
    println!("  > /model GEMINI_1_5_PRO_002");
    println!("  > /export ~/yappus_backup.json");
}
