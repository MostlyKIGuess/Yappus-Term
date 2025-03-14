use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;

pub fn get_config_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "yappus", "yappus-term") {
        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir).unwrap();
        config_dir.to_path_buf()
    } else {
        PathBuf::from(".")
    }
}

pub fn render_response(text: &str) -> String {
    let mut formatted = text.replace("```bash", "\x1b[1;33m"); // Yellow for bash code
    formatted = formatted.replace("```shell", "\x1b[1;33m"); // Yellow for shell code
    formatted = formatted.replace("```javascript", "\x1b[1;34m"); // Blue for JavaScript
    formatted = formatted.replace("```python", "\x1b[1;32m"); // Green for Python
    formatted = formatted.replace("```rust", "\x1b[1;31m"); // Red for Rust
    formatted = formatted.replace("```", "\x1b[0m"); // Reset color at end of code block
    
    formatted
}

pub fn set_model(model_name: &str, config_dir: &PathBuf) -> bool {
    let config_file = config_dir.join("config.json");
    let model = match model_name.to_uppercase().as_str() {
        "GEMINI_1_5_FLASH" => "GEMINI_1_5_FLASH",
        "GEMINI_1_5_PRO_002" => "GEMINI_1_5_PRO_002",
        "GEMINI_1_5_PRO" => "GEMINI_1_5_PRO",
        "GEMINI_1_5_FLASH_002" => "GEMINI_1_5_FLASH_002",
        "GEMINI_1_5_FLASH_8B" => "GEMINI_1_5_FLASH_8B",
        "GEMINI_1_0_PRO" => "GEMINI_1_0_PRO",
        _ => {
            println!("Invalid model name. Using default GEMINI_1_5_FLASH");
            "GEMINI_1_5_FLASH"
        }
    };
    
    // Write to config file
    let config = serde_json::json!({
        "model": model
    });
    
    match std::fs::write(config_file, serde_json::to_string_pretty(&config).unwrap()) {
        Ok(_) => {
            println!("Model set to: {}", model);
            true
        },
        Err(e) => {
            eprintln!("Failed to save model config: {}", e);
            false
        }
    }
}

pub fn view_history(history_path: &str) {
    use crate::memory;
    
    let chat_history = memory::load_chat(history_path);
    if chat_history.is_empty() {
        println!("No chat history found.");
        return;
    }
    
    println!("\n--- Chat History ---");
    for (idx, entry) in chat_history.iter().enumerate() {
        println!("Chat #{}", idx + 1);
        println!("You: {}", entry.user);
        println!("Gemini: {}\n", entry.bot);
    }
}

pub fn clear_history(history_path: &str) -> bool {
    match std::fs::write(history_path, "") {
        Ok(_) => {
            println!("Chat history cleared.");
            true
        },
        Err(e) => {
            eprintln!("Failed to clear history: {}", e);
            false
        }
    }
}

pub fn display_config(config_dir: &PathBuf) {
    let config_file = config_dir.join("config.json");
    
    println!("\n=== Yappus Configuration ===");
    println!("Configuration directory: {}", config_dir.display());
    
    if let Ok(config_data) = fs::read_to_string(&config_file) {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_data) {
            if let Some(model_name) = config["model"].as_str() {
                println!("Current model: {}", model_name);
            }
        }
    } else {
        println!("No configuration file found or unable to read it.");
    }
    
    let history_path = config_dir.join("chat_history.json");
    if history_path.exists() {
        if let Ok(metadata) = fs::metadata(&history_path) {
            let size_kb = metadata.len() / 1024;
            println!("Chat history size: {} KB", size_kb);
        }
    } else {
        println!("No chat history found.");
    }
}

pub fn export_history(history_path: &str, export_path: &str) {
    match fs::copy(history_path, export_path) {
        Ok(_) => println!("Chat history exported to: {}", export_path),
        Err(e) => eprintln!("Failed to export history: {}", e),
    }
}
