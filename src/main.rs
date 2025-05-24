use clap::{Parser, Subcommand};
use memory::ChatLog;
use std::path::Path;

mod api;
mod cli;
mod memory;
mod startup;
mod terminal;
mod ui;
mod utils;

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
    let history_path = config_dir
        .join("chat_history.json")
        .to_str()
        .unwrap()
        .to_string();

    let is_first_run = !config_dir.join("config.json").exists();
    if is_first_run {
        startup::initial_setup(&config_dir);
    } else {
        std::fs::create_dir_all(&config_dir).unwrap_or(());
    }
    let api_key = startup::get_api_key(&config_dir);

    let cli = Cli::parse();

    match cli.command {
        Some(command) => cli::handle_cli_command(command, &config_dir, &history_path, &api_key),
        None => {
            if !cli.query.is_empty() {
                let query = cli.query.join(" ");
                process_query(&query, &api_key, &config_dir, &history_path);
            } else {
                terminal::interactive_mode(&api_key, &config_dir, &history_path);
            }
        }
    }
}

fn process_query(query: &str, api_key: &str, config_dir: &Path, history_path: &str) {
    let recent_history = memory::get_recent_history(history_path, 5);

    let context_query = if !recent_history.is_empty() {
        format!(
            "Previous conversations for context:\n{}\n\nNew query: {}",
            recent_history, query
        )
    } else {
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
