use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn change_directory(path: &str) {
    let expanded_path = if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path == "~" {
                home
            } else {
                home.join(&path[2..])
            }
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    };

    match env::set_current_dir(&expanded_path) {
        Ok(_) => {
            println!(
                "{}",
                format!("Changed directory to: {}", expanded_path.display()).green()
            );
        }
        Err(e) => {
            println!("{}", format!("Error changing directory: {}", e).red());
        }
    }
}

pub fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display().to_string().bright_blue());
        }
        Err(e) => {
            println!(
                "{}",
                format!("Error getting current directory: {}", e).red()
            );
        }
    }
}

pub fn list_directory(path: &str) {
    let expanded_path = if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path == "~" {
                home
            } else {
                home.join(&path[2..])
            }
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    };

    match std::fs::read_dir(&expanded_path) {
        Ok(entries) => {
            println!(
                "{}",
                format!("Contents of '{}':", expanded_path.display())
                    .bright_yellow()
                    .bold()
            );
            println!();

            let mut files = Vec::new();
            let mut dirs = Vec::new();

            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.path().is_dir() {
                    dirs.push(name);
                } else {
                    files.push(name);
                }
            }

            dirs.sort();
            files.sort();

            for dir in dirs {
                println!("  {} {}/", "[DIR]".blue().bold(), dir.blue());
            }

            for file in files {
                let file_type = get_file_type(&file);
                println!("  {} {}", file_type.bright_black(), file.white());
            }

            if expanded_path != PathBuf::from(".") {
                println!();
                println!(
                    "{}",
                    "Tip: Use /file <filename> to analyze a file".bright_black()
                );
            }
        }
        Err(e) => println!("{}", format!("Error reading directory: {}", e).red()),
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    display_welcome_banner();
}

pub fn display_welcome_banner() {
    println!();
    println!(
        "{}",
        "=========================================".bright_cyan()
    );
    println!(
        "{}",
        "    Welcome to Yappus Terminal v1.1.0   ".bright_cyan()
    );
    println!(
        "{}",
        "   Your AI assistant that YAPS!HEHEHEHA ".bright_cyan()
    );
    println!(
        "{}",
        "=========================================".bright_cyan()
    );
    println!();
    println!("{}", "Tips:".bright_yellow().bold());
    println!("  * Type your question directly");
    println!(
        "  * Use {} for commands (try typing {} and press TAB)",
        "/".cyan(),
        "/".cyan()
    );
    println!(
        "  * Press {} for auto-completion",
        "TAB".bright_green().bold()
    );
    println!("  * Use {} or {} to exit", "exit".red(), "Ctrl+D".red());
    println!("  * Type {} for help", "/help".cyan());
    println!(
        "  * Pipe commands: {} or {}",
        "/ls | what programming languages are here?".bright_green(),
        "/pwd | what should I do in this directory?".bright_green()
    );
    println!();
}

fn get_file_type(filename: &str) -> String {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match ext.to_lowercase().as_str() {
        "rs" => "[RUST]".to_string(),
        "py" => "[PYTHON]".to_string(),
        "js" | "ts" => "[JS/TS]".to_string(),
        "html" => "[HTML]".to_string(),
        "css" => "[CSS]".to_string(),
        "json" => "[JSON]".to_string(),
        "md" => "[MARKDOWN]".to_string(),
        "txt" => "[TEXT]".to_string(),
        "pdf" => "[PDF]".to_string(),
        "png" | "jpg" | "jpeg" | "gif" => "[IMAGE]".to_string(),
        "zip" | "tar" | "gz" => "[ARCHIVE]".to_string(),
        "toml" => "[TOML]".to_string(),
        "yaml" | "yml" => "[YAML]".to_string(),
        "sh" => "[SHELL]".to_string(),
        "" => "[FILE]".to_string(),
        _ => format!("[{}]", ext.to_uppercase()),
    }
}
