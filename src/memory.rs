use std::fs::{OpenOptions, File};
use std::io::{BufReader, Write, BufRead};
use serde::{Serialize, Deserialize};

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

    reader.lines()
        .filter_map(|line| serde_json::from_str::<ChatLog>(&line.unwrap()).ok())
        .collect()
}
