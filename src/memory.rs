use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize, Deserialize)]
pub struct ChatLog {
    pub user: String,
    pub bot: String,
}

pub fn save_chat(log: &ChatLog, file_path: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Unable to open file");

    let log_entry = serde_json::to_string(log).unwrap();
    writeln!(file, "{}", log_entry).expect("Unable to write to file");
}

pub fn load_chat(file_path: &str) -> Vec<ChatLog> {
    let file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| serde_json::from_str::<ChatLog>(&line.unwrap()).ok())
        .collect()
}

pub fn get_recent_history(history_path: &str, limit: usize) -> String {
    let chat_history = load_chat(history_path);
    if chat_history.is_empty() {
        return String::new();
    }
    let recent_chats = chat_history.iter().rev().take(limit).rev();

    let mut result = String::new();
    for (idx, entry) in recent_chats.enumerate() {
        result.push_str(&format!("--- Previous Conversation #{} ---\n", idx + 1));
        result.push_str(&format!("User: {}\n", entry.user));
        result.push_str(&format!("AI: {}\n\n", entry.bot));
    }

    result
}
