use rustyline::{
    completion::{Completer, FilenameCompleter, Pair},
    Context,
};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct YappusCompleter {
    file_completer: FilenameCompleter,
    commands: HashSet<String>,
    models: HashSet<String>,
}

impl YappusCompleter {
    pub fn new() -> Self {
        let mut commands = HashSet::new();
        commands.insert("/help".to_string());
        commands.insert("/model".to_string());
        commands.insert("/history".to_string());
        commands.insert("/clearhistory".to_string());
        commands.insert("/setup".to_string());
        commands.insert("/version".to_string());
        commands.insert("/config".to_string());
        commands.insert("/key".to_string());
        commands.insert("/export".to_string());
        commands.insert("/file".to_string());
        commands.insert("/ls".to_string());
        commands.insert("/cd".to_string());
        commands.insert("/pwd".to_string());
        commands.insert("/clear".to_string());
        commands.insert("exit".to_string());
        commands.insert("quit".to_string());

        let mut models = HashSet::new();
        models.insert("GEMINI_FLASH".to_string());
        models.insert("GEMINI_2_5_FLASH".to_string());
        models.insert("GEMINI_2_5_PRO".to_string());
        models.insert("GEMINI_1_5_FLASH".to_string());
        models.insert("GEMINI_1_5_PRO".to_string());

        YappusCompleter {
            file_completer: FilenameCompleter::new(),
            commands,
            models,
        }
    }
}

impl Completer for YappusCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let trimmed = line.trim_start();

        // Command completion - only at the beginning
        if (trimmed.starts_with('/') && !trimmed.contains(' '))
            || trimmed == "exit"
            || trimmed == "quit"
        {
            let mut matches = Vec::new();
            for cmd in &self.commands {
                if cmd.starts_with(trimmed) {
                    matches.push(Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    });
                }
            }
            return Ok((0, matches));
        }

        // Directory completion for /cd and /ls
        if trimmed.starts_with("/cd ") || trimmed.starts_with("/ls ") {
            return self.complete_directory(trimmed, pos);
        }

        // Model completion after /model command
        if let Some(model_part) = trimmed.strip_prefix("/model ") {
            let model_start = 7; // Length of "/model "
            let mut matches = Vec::new();
            for model in &self.models {
                if model.to_lowercase().starts_with(&model_part.to_lowercase()) {
                    matches.push(Pair {
                        display: model.clone(),
                        replacement: model.clone(),
                    });
                }
            }
            return Ok((model_start, matches));
        }

        // File completion for /file and /export commands
        if trimmed.starts_with("/file ") || trimmed.starts_with("/export ") {
            return self.file_completer.complete(line, pos, ctx);
        }

        Ok((pos, vec![]))
    }
}

impl YappusCompleter {
    fn complete_directory(
        &self,
        trimmed: &str,
        _pos: usize,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let cmd_len = 4; // Both "/cd " and "/ls " are 4 characters
        let path_start = cmd_len;
        let path_part = &trimmed[cmd_len..];

        let mut matches = Vec::new();
        let search_dir = if path_part.is_empty() {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        } else {
            let path = PathBuf::from(path_part);
            if path.is_absolute() {
                if path.is_dir() {
                    path
                } else {
                    path.parent().unwrap_or(&path).to_path_buf()
                }
            } else {
                let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                if path_part.contains('/') {
                    current.join(path.parent().unwrap_or(&path))
                } else {
                    current
                }
            }
        };

        if let Ok(entries) = std::fs::read_dir(&search_dir) {
            let filter_prefix = if path_part.contains('/') {
                path_part.split('/').next_back().unwrap_or("")
            } else {
                path_part
            };

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with(filter_prefix) && !name.starts_with('.') {
                        let replacement = if path_part.contains('/') {
                            let base = &path_part[..path_part.rfind('/').unwrap() + 1];
                            format!("{}{}/", base, name)
                        } else {
                            format!("{}/", name)
                        };

                        matches.push(Pair {
                            display: format!("{}/", name),
                            replacement,
                        });
                    }
                }
            }
        }

        let filter_prefix = if path_part.contains('/') {
            path_part.split('/').next_back().unwrap_or("")
        } else {
            path_part
        };

        if filter_prefix.is_empty() || "..".starts_with(filter_prefix) {
            matches.push(Pair {
                display: "../".to_string(),
                replacement: "../".to_string(),
            });
        }

        Ok((path_start, matches))
    }
}
