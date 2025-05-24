use colored::*;
use rustyline::{
    error::ReadlineError,
    highlight::{Highlighter, MatchingBracketHighlighter},
    hint::{Hinter, HistoryHinter},
    validate::{self, MatchingBracketValidator, Validator},
    Config, Context, Editor, Helper,
};
use std::borrow::Cow::{self, Borrowed, Owned};
use std::env;
use std::path::PathBuf;

use super::completion::YappusCompleter;
use super::shell_commands::{self, display_welcome_banner};

pub struct YappusHelper {
    completer: YappusCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl YappusHelper {
    pub fn new() -> Self {
        let colored_prompt = format!("{} ", ">".bright_green().bold());
        YappusHelper {
            completer: YappusCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: HistoryHinter {},
            colored_prompt,
        }
    }
}

impl rustyline::completion::Completer for YappusHelper {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<rustyline::completion::Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for YappusHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for YappusHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("{}", hint.bright_black()))
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let mut highlighted = String::new();
        let trimmed = line.trim_start();

        if trimmed.starts_with('/') {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            highlighted.push_str(&format!("{}", parts[0].cyan().bold()));
            if parts.len() > 1 {
                highlighted.push(' ');
                highlighted.push_str(parts[1]);
            }
            Owned(highlighted)
        } else if trimmed == "exit" || trimmed == "quit" {
            Owned(format!("{}", line.red().bold()))
        } else {
            self.highlighter.highlight(line, _pos)
        }
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

impl Validator for YappusHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

impl Helper for YappusHelper {}

pub fn interactive_mode(api_key: &str, config_dir: &PathBuf, history_path: &str) {
    display_welcome_banner();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .build();

    let helper = YappusHelper::new();
    let mut rl = Editor::with_config(config).expect("Failed to create readline editor");
    rl.set_helper(Some(helper));

    let history_file = config_dir.join("readline_history.txt");
    if history_file.exists() {
        let _ = rl.load_history(&history_file);
    }

    loop {
        let prompt = format!("{} ", ">".bright_green().bold());
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(input).unwrap_or_default();

                if input == "exit" || input == "quit" {
                    println!("{}", "Goodbye!".bright_yellow());
                    break;
                }

                // Check for piped commands
                if let Some(pipe_pos) = input.find(" | ") {
                    handle_piped_command(input, pipe_pos, api_key, config_dir, history_path);
                } else if input.starts_with('/') {
                    handle_interactive_command(input, api_key, config_dir, history_path);
                } else {
                    crate::process_query(input, api_key, config_dir, history_path);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "Use 'exit' or Ctrl+D to quit".yellow());
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".bright_yellow());
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(&history_file);
}

fn handle_piped_command(
    input: &str,
    pipe_pos: usize,
    api_key: &str,
    config_dir: &PathBuf,
    history_path: &str,
) {
    let command_part = input[..pipe_pos].trim();
    let query_part = input[pipe_pos + 3..].trim(); // Skip " | "

    let output = match command_part {
        "/ls" => get_directory_listing("."),
        cmd if cmd.starts_with("/ls ") => {
            let path = &cmd[4..];
            get_directory_listing(path)
        }
        "/pwd" => get_current_directory(),
        _ => {
            println!(
                "{}",
                "Only /ls and /pwd commands can be piped to AI".yellow()
            );
            return;
        }
    };

    if let Some(output) = output {
        let full_query = format!(
            "Based on this command output:\n\n{}\n\nQuestion: {}",
            output, query_part
        );
        crate::process_query(&full_query, api_key, config_dir, history_path);
    }
}

fn get_directory_listing(path: &str) -> Option<String> {
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
            let mut output = format!("Directory listing for '{}':\n", expanded_path.display());
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

            output.push_str("\nDirectories:\n");
            for dir in dirs {
                output.push_str(&format!("  {}/\n", dir));
            }

            output.push_str("\nFiles:\n");
            for file in files {
                output.push_str(&format!("  {}\n", file));
            }

            Some(output)
        }
        Err(_) => None,
    }
}

fn get_current_directory() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(format!("Current directory: {}", path.display())),
        Err(_) => None,
    }
}

fn handle_interactive_command(
    command: &str,
    api_key: &str,
    config_dir: &PathBuf,
    history_path: &str,
) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let cmd = parts[0];

    match cmd {
        "/help" => crate::ui::HelpFormatter::display_full_help(),
        "/model" => {
            if parts.len() > 1 {
                crate::utils::set_model(parts[1], config_dir);
            } else {
                display_available_models_interactive();
            }
        }
        "/history" => crate::utils::view_history(history_path),
        "/clearhistory" => {
            let _ = crate::utils::clear_history(history_path);
            println!("{}", "Chat history cleared successfully!".green());
        }
        "/setup" => {
            crate::startup::initial_setup(config_dir);
            println!("{}", "Setup completed!".green());
        }
        "/version" => crate::startup::display_version(),
        "/config" => crate::utils::display_config(config_dir),
        "/key" => {
            if parts.len() > 1 && parts[1] == "reset" {
                crate::startup::reset_api_key(config_dir);
            } else {
                crate::startup::check_api_key(config_dir);
            }
        }
        "/export" => {
            let export_path = if parts.len() > 1 {
                parts[1].to_string()
            } else {
                "yappus_history_export.json".to_string()
            };
            crate::utils::export_history(history_path, &export_path);
        }
        "/file" => {
            if parts.len() > 1 {
                let file_path = parts[1];
                let query: Vec<String> = parts[2..].iter().map(|s| s.to_string()).collect();
                handle_file_command_interactive(
                    file_path,
                    &query,
                    api_key,
                    config_dir,
                    history_path,
                );
            } else {
                println!("{}", "Usage: /file <path> [query]".yellow());
            }
        }
        "/ls" => {
            let path = if parts.len() > 1 { parts[1] } else { "." };
            shell_commands::list_directory(path);
        }
        "/cd" => {
            let path = if parts.len() > 1 {
                parts[1]
            } else if let Some(home) = dirs::home_dir() {
                return shell_commands::change_directory(&home.to_string_lossy());
            } else {
                return shell_commands::change_directory(".");
            };
            shell_commands::change_directory(path);
        }
        "/pwd" => {
            shell_commands::print_working_directory();
        }
        "/clear" => {
            shell_commands::clear_screen();
        }
        _ => {
            println!(
                "{}",
                format!(
                    "Unknown command: {}. Type /help for available commands.",
                    cmd
                )
                .red()
            );
        }
    }
}

fn display_available_models_interactive() {
    println!("{}", "Available Models:".bright_yellow().bold());
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
    println!("{}", "Usage: /model <MODEL_NAME>".cyan());
    println!("{}", "Example: /model GEMINI_2_5_PRO".bright_white());
}

fn handle_file_command_interactive(
    path: &str,
    query: &[String],
    api_key: &str,
    config_dir: &PathBuf,
    history_path: &str,
) {
    println!("{}", format!("Reading content from: {}", path).cyan());

    match crate::utils::read_file_content(path) {
        Ok(content) => {
            let user_query = if !query.is_empty() {
                query.join(" ")
            } else {
                "Explain this file".to_string()
            };

            let sanitized = crate::utils::sanitize_file_content(&content)
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

            crate::process_query(&file_query, api_key, config_dir, history_path);
        }
        Err(e) => println!("{}", format!("Error reading file: {}", e).red()),
    }
}
