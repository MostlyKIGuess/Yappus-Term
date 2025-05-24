use crate::{process_query, startup, utils, Commands};
use colored::*;
use std::path::PathBuf;

pub fn handle_cli_command(
    command: Commands,
    config_dir: &PathBuf,
    history_path: &str,
    api_key: &str,
) {
    match command {
        Commands::Model { name } => {
            if let Some(model_name) = name {
                utils::set_model(&model_name, config_dir);
            } else {
                display_available_models();
            }
        }
        Commands::History => utils::view_history(history_path),
        Commands::ClearHistory => {
            let _ = utils::clear_history(history_path);
            println!("{}", "Chat history cleared successfully!".green().bold());
        }
        Commands::Setup => {
            startup::initial_setup(config_dir);
            println!(
                "{}",
                "Setup completed. Run 'yappus' to start using the application."
                    .green()
                    .bold()
            );
        }
        Commands::Version => startup::display_version(),
        Commands::Config => utils::display_config(config_dir),
        Commands::Key { reset } => {
            if reset {
                startup::reset_api_key(config_dir);
            } else {
                startup::check_api_key(config_dir);
            }
        }
        Commands::Export { path } => {
            let export_path = path.unwrap_or_else(|| "yappus_history_export.json".to_string());
            utils::export_history(history_path, &export_path);
        }
        Commands::File { path, query } => {
            handle_file_command(&path, &query, api_key, config_dir, history_path);
        }
    }
}

fn display_available_models() {
    println!();
    println!("{}", "Available Gemini Models:".bright_yellow().bold());
    println!();

    let models = vec![
        ("GEMINI_FLASH", "Latest Gemini 2.0 model (default)"),
        ("GEMINI_2_5_PRO", "Most powerful for complex tasks"),
        (
            "GEMINI_2_5_FLASH",
            "High performance with excellent reasoning",
        ),
        ("GEMINI_1_5_PRO", "Very capable legacy model"),
        ("GEMINI_1_5_FLASH", "Fast and efficient responses"),
    ];

    for (name, desc) in models {
        println!("  {:20} {}", name.bright_green().bold(), desc.white());
    }

    println!();
    println!("{}", "Usage:".bright_yellow().bold());
    println!(
        "  {} {}",
        "yappus model GEMINI_2_5_PRO".cyan(),
        "# Change to most powerful model".bright_black()
    );
    println!(
        "  {} {}",
        "yappus model".cyan(),
        "# Show this list".bright_black()
    );
    println!();
}

fn handle_file_command(
    path: &str,
    query: &[String],
    api_key: &str,
    config_dir: &PathBuf,
    history_path: &str,
) {
    println!(
        "{}",
        format!("Reading content from: {}", path).cyan().bold()
    );

    match utils::read_file_content(path) {
        Ok(content) => {
            let user_query = if !query.is_empty() {
                query.join(" ")
            } else {
                "Explain this file".to_string()
            };

            let sanitized = utils::sanitize_file_content(&content)
                .replace('"', "\\\"")
                .replace('\'', "");

            let truncated_content = if sanitized.len() > 4000 {
                println!(
                    "{}",
                    "Warning: File content truncated to 4000 characters".yellow()
                );
                format!("{}... (truncated)", &sanitized[..4000])
            } else {
                sanitized
            };

            let file_query = format!(
                "File '{}' contains:\n\n{}\n\nQuestion about this file: {}",
                path, truncated_content, user_query
            );

            process_query(&file_query, api_key, config_dir, history_path);
        }
        Err(e) => println!("{}", format!("Error reading file: {}", e).red().bold()),
    }
}

