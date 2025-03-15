use clap::{Parser, Subcommand};
use std::io::{self, Write};
use crate::memory::ChatLog;
use std::path::PathBuf;

mod memory;
mod api;
mod utils;
mod startup;

#[derive(Parser)]
#[command(name = "yappus")]
#[command(author = "MostlyK <bruvistrue93@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A terminal interface for your AI terminal assistant, warning it YAPS!", long_about = None)]
struct Cli {
    query: Vec<String>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Model {
        name: Option<String>,
    },
    
    History,    
    ClearHistory,
    Setup,    
    Version,    
    Config,    
    Key {
        reset: bool,
    },    
    Export {
        path: Option<String>,
    },
    File {
        path: String,
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        query: Vec<String>,
    },
}

fn main() {
    dotenvy::dotenv().ok();
    
    let config_dir = utils::get_config_dir();
    let history_path = config_dir.join("chat_history.json").to_str().unwrap().to_string();
    
    let is_first_run = !config_dir.join("config.json").exists();
    if is_first_run {
        startup::initial_setup(&config_dir);
    } else {
        std::fs::create_dir_all(&config_dir).unwrap_or(());
    }   
    let api_key = startup::get_api_key(&config_dir);
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Model { name }) => {
            if let Some(model_name) = name {
                utils::set_model(&model_name, &config_dir);
            } else {
                println!("Available models:");
                println!("  - GEMINI_1_5_FLASH (default)");
                println!("  - GEMINI_1_5_PRO_002");
                println!("  - GEMINI_1_5_PRO");
                println!("  - GEMINI_1_5_FLASH_002");
                println!("  - GEMINI_1_5_FLASH_8B");
                println!("  - GEMINI_1_0_PRO");
                println!("\nExample usage:");
                println!("  yappus model GEMINI_1_5_PRO");
            }
        },
        Some(Commands::History) => {
            utils::view_history(&history_path);
        },
        Some(Commands::ClearHistory) => {
            let _ = utils::clear_history(&history_path);
        },
        Some(Commands::Setup) => {
            startup::initial_setup(&config_dir);
            println!("Setup completed. Run 'yappus' to start using the application.");
        },
        Some(Commands::Version) => {
            startup::display_version();
        },
        Some(Commands::Config) => {
            utils::display_config(&config_dir);
        },
        Some(Commands::Key { reset }) => {
            if reset {
                startup::reset_api_key(&config_dir);
            } else {
                startup::check_api_key(&config_dir);
            }
        },
        Some(Commands::Export { path }) => {
            let export_path = path.unwrap_or_else(|| "yappus_history_export.json".to_string());
            utils::export_history(&history_path, &export_path);
        },
        Some(Commands::File { path, query }) => {
            println!("Reading content from: {}", path);
            match utils::read_file_content(&path) {
                Ok(content) => {
                    let user_query = if !query.is_empty() {
                        query.join(" ")
                    } else {
                        "Explain this file".to_string()
                    };
                    
                    // sanitization !! Very important
                    let sanitized = utils::sanitize_file_content(&content)
                        .replace('"', "\\\"")  // escape double quotes
                        .replace('\'', "");    // remove single quotes
                    
                    let truncated_content = if sanitized.len() > 4000 {
                        format!("{}... (truncated)", &sanitized[..4000])
                    } else {
                        sanitized
                    };
                    
                    let file_query = format!(
                        "File '{}' contains:\n\n{}\n\nQuestion about this file: {}", 
                        path, truncated_content, user_query
                    );
                    
                    process_query(&file_query, &api_key, &config_dir, &history_path);
                },
                Err(e) => println!("Error reading file: {}", e),
            }
        },
        None => {
            if !cli.query.is_empty() {
                let query = cli.query.join(" ");
                process_query(&query, &api_key, &config_dir, &history_path);
            } else {
                interactive_mode(&api_key, &config_dir, &history_path);
            }
        }
    }
}

fn process_query(query: &str, api_key: &str, config_dir: &PathBuf, history_path: &str) {

    let recent_history = memory::get_recent_history(history_path, 5);

    let context_query = if !recent_history.is_empty() {
        format!("Previous conversations for context:\n{}\n\nNew query: {}", recent_history, query)
    } else{
        query.to_string()
    };

    match api::get_gemini_response(&context_query, api_key, config_dir) {
        Ok(response) => {
            let formatted_response = utils::render_response(&response);
            println!("{}", formatted_response);
            
            let log_entry = ChatLog {
                user: query.to_string(),
                bot: response.clone(),
            };
            memory::save_chat(&log_entry, history_path);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn interactive_mode(api_key: &str, config_dir: &PathBuf, history_path: &str) {
    println!("Welcome to Yappus Terminal! Type 'exit' to quit or '/help' for commands.");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }
        
        if input.starts_with("/") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            match parts[0] {
                "/model" => {
                    if parts.len() > 1 {
                        utils::set_model(parts[1], config_dir);
                    } else {
                        println!("Available models:");
                        println!("  - GEMINI_1_5_FLASH (default)");
                        println!("  - GEMINI_1_5_PRO_002");
                        println!("  - GEMINI_1_5_PRO");
                        println!("  - GEMINI_1_5_FLASH_002");
                        println!("  - GEMINI_1_5_FLASH_8B");
                        println!("  - GEMINI_1_0_PRO");
                    }
                },
                "/history" => utils::view_history(history_path),
                "/clearhistory" => { let _ = utils::clear_history(history_path); },
                "/help" => startup::display_help(),
                "/setup" => { startup::initial_setup(config_dir); },
                "/config" => { utils::display_config(config_dir); },
                "/version" => { startup::display_version(); },
                "/key" => {
                    if parts.len() > 1 && parts[1] == "reset" {
                        startup::reset_api_key(config_dir);
                    } else {
                        startup::check_api_key(config_dir);
                    }
                },
                "/export" => {
                    let export_path = if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        "yappus_history_export.json".to_string()
                    };
                    utils::export_history(history_path, &export_path);
                },
                "/clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                },
                "/ls" => {
                    let dir_path = if parts.len() > 1 { Some(parts[1]) } else { None };
                    match utils::list_directory_files(dir_path) {
                        Ok(entries) => {
                            let current_dir = if let Some(path) = dir_path {
                                std::path::PathBuf::from(path)
                            } else {
                                std::env::current_dir().unwrap_or_default()
                            };
                            println!("Contents of {}:", current_dir.display());
                            
                            println!("Directories:");
                            for entry in &entries {
                                if entry.is_dir() {
                                    let name = entry.file_name().unwrap_or_default().to_string_lossy();
                                    println!("  📁 {}", name);
                                }
                            }
                            
                            println!("Files:");
                            for entry in &entries {
                                if entry.is_file() {
                                    let name = entry.file_name().unwrap_or_default().to_string_lossy();
                                    let size = match entry.metadata() {
                                        Ok(meta) => format!("{} KB", meta.len() / 1024),
                                        Err(_) => "? KB".to_string(),
                                    };
                                    println!("  📄 {:<20} {}", name, size);
                                }
                            }
                        },
                        Err(e) => println!("Error listing directory: {}", e),
                    }
                },
                "/file" => {
                    if parts.len() >= 2 {
                        let file_path = parts[1];
                        println!("Reading content from: {}", file_path);
                        match utils::read_file_content(file_path) {
                            Ok(content) => {
                                let user_query = if parts.len() > 2 {
                                    parts[2..].join(" ")
                                } else {
                                    "Explain this file".to_string()
                                };
                                
                                // sanitize and escape quotes
                                let sanitized = utils::sanitize_file_content(&content)
                                    .replace('"', "\\\"") // escqpe double quotes
                                    .replace('\'', "");   // remove single quotes as well

                                let truncated_content = if sanitized.len() > 4000 {
                                    format!("{}... (truncated)", &sanitized[..4000])
                                } else {
                                    sanitized
                                };
                                
                                let file_query = format!(
                                    "File '{}' contains:\n\n{}\n\nQuestion about this file: {}", 
                                    file_path, truncated_content, user_query
                                );

                                println!("{}", file_query);
                                process_query(&file_query, api_key, config_dir, history_path);
                            },
                            Err(e) => println!("Error reading file: {}", e),
                        }
                    } else {
                        println!("Usage: /file <file_path> [question about file]");
                    }
                },
                _ => println!("Unknown command. Type /help for available commands.")
            }
            continue;
        }

        process_query(input, api_key, config_dir, history_path);
    }
}
