use directories::ProjectDirs;
use pulldown_cmark::{Options, Parser};
use std::fs;
use std::path::{Path, PathBuf};

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
    render_markdown(text)
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    let mut in_code_block = false;
    let mut code_language: String = String::new();
    let mut code_content = String::new();
    let mut list_depth: usize = 0;

    for event in parser {
        match event {
            pulldown_cmark::Event::Start(tag) => {
                match tag {
                    pulldown_cmark::Tag::Heading(level, _, _) => {
                        output.push('\n');
                        let (color, symbol) = match level {
                            pulldown_cmark::HeadingLevel::H1 => ("\x1b[1;96m", "█"), // Bright cyan
                            pulldown_cmark::HeadingLevel::H2 => ("\x1b[1;95m", "▓"), // Bright magenta
                            pulldown_cmark::HeadingLevel::H3 => ("\x1b[1;94m", "▒"), // Bright blue
                            pulldown_cmark::HeadingLevel::H4 => ("\x1b[1;93m", "░"), // Bright yellow
                            _ => ("\x1b[1;97m", "▪"),                                // Bright white
                        };
                        output.push_str(&format!("{}{} ", color, symbol));
                    }
                    pulldown_cmark::Tag::Strong => {
                        output.push_str("\x1b[1;97m"); // Bright white bold
                    }
                    pulldown_cmark::Tag::Emphasis => {
                        output.push_str("\x1b[3;96m"); // Italic cyan
                    }
                    pulldown_cmark::Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_content.clear();

                        if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                            code_language = lang.to_string();
                        } else {
                            code_language.clear();
                        }

                        output.push('\n');
                        // Top border
                        output.push_str("\x1b[90m┌");
                        if !code_language.is_empty() {
                            output.push_str(&format!("─ \x1b[93m{}\x1b[90m ", code_language));
                            output.push_str(
                                &"─".repeat(60_usize.saturating_sub(code_language.len() + 3)),
                            );
                        } else {
                            output.push_str(&"─".repeat(60));
                        }
                        output.push_str("┐\x1b[0m\n");
                    }
                    pulldown_cmark::Tag::List(_) => {
                        if list_depth == 0 {
                            output.push('\n');
                        }
                        list_depth += 1;
                    }
                    pulldown_cmark::Tag::Item => {
                        let indent = "  ".repeat(list_depth.saturating_sub(1));
                        output.push_str(&format!("{}∘ ", indent));
                    }
                    pulldown_cmark::Tag::BlockQuote => {
                        output.push_str("\x1b[90m▐ \x1b[3;37m"); // Gray bar with italic text
                    }
                    pulldown_cmark::Tag::Link(_, _, _) => {
                        output.push_str("\x1b[4;34m"); // Blue underlined
                    }
                    pulldown_cmark::Tag::Strikethrough => {
                        output.push_str("\x1b[9m"); // Strikethrough
                    }
                    pulldown_cmark::Tag::Table(_) => {
                        output.push('\n');
                        output.push_str("\x1b[90m┌─ Table ─┐\x1b[0m\n");
                    }
                    pulldown_cmark::Tag::TableHead => {
                        output.push_str("\x1b[1;93m"); // Bold yellow headers
                    }
                    pulldown_cmark::Tag::TableCell => {
                        output.push_str("│ ");
                    }
                    _ => {}
                }
            }
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::Tag::Heading(level, _, _) => {
                    output.push_str("\x1b[0m");
                    match level {
                        pulldown_cmark::HeadingLevel::H1 => {
                            output.push('\n');
                            output.push_str("\x1b[90m");
                            output.push_str(&"═".repeat(60));
                            output.push_str("\x1b[0m");
                        }
                        pulldown_cmark::HeadingLevel::H2 => {
                            output.push('\n');
                            output.push_str("\x1b[90m");
                            output.push_str(&"─".repeat(40));
                            output.push_str("\x1b[0m");
                        }
                        _ => {}
                    }
                    output.push('\n');
                }
                pulldown_cmark::Tag::Strong
                | pulldown_cmark::Tag::Emphasis
                | pulldown_cmark::Tag::Strikethrough
                | pulldown_cmark::Tag::Link(_, _, _) => {
                    output.push_str("\x1b[0m");
                }
                pulldown_cmark::Tag::CodeBlock(_) => {
                    in_code_block = false;

                    if !code_content.is_empty() {
                        let highlighted = highlight_code_block(&code_content, &code_language);
                        for line in highlighted.lines() {
                            output.push_str("\x1b[90m│\x1b[0m ");
                            output.push_str(line);
                            output.push('\n');
                        }
                    }

                    output.push_str("\x1b[90m└");
                    output.push_str(&"─".repeat(60));
                    output.push_str("┘\x1b[0m\n");
                }
                pulldown_cmark::Tag::List(_) => {
                    list_depth = list_depth.saturating_sub(1);
                    if list_depth == 0 {
                        output.push('\n');
                    }
                }
                pulldown_cmark::Tag::Item => {
                    output.push('\n');
                }
                pulldown_cmark::Tag::Paragraph => {
                    if !in_code_block {
                        output.push('\n');
                    }
                }
                pulldown_cmark::Tag::BlockQuote => {
                    output.push_str("\x1b[0m\n");
                }
                pulldown_cmark::Tag::TableHead => {
                    output.push_str("\x1b[0m\n");
                    output.push_str("\x1b[90m");
                    output.push_str(&"─".repeat(60));
                    output.push_str("\x1b[0m\n");
                }
                pulldown_cmark::Tag::TableRow => {
                    output.push('\n');
                }
                _ => {}
            },
            pulldown_cmark::Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else {
                    output.push_str(&text);
                }
            }
            pulldown_cmark::Event::Code(code) => {
                output.push_str("\x1b[48;5;238;38;5;156m");
                output.push_str(" ");
                output.push_str(&code);
                output.push_str(" ");
                output.push_str("\x1b[0m");
            }
            pulldown_cmark::Event::SoftBreak => {
                if in_code_block {
                    code_content.push('\n');
                } else {
                    output.push(' ');
                }
            }
            pulldown_cmark::Event::HardBreak => {
                if in_code_block {
                    code_content.push('\n');
                } else {
                    output.push('\n');
                }
            }
            pulldown_cmark::Event::TaskListMarker(checked) => {
                if checked {
                    output.push_str("\x1b[92m✓\x1b[0m ");
                } else {
                    output.push_str("\x1b[90m○\x1b[0m ");
                }
            }
            _ => {}
        }
    }

    output
}

fn highlight_code_block(code: &str, language: &str) -> String {
    if !language.is_empty() {
        if let Ok(highlighted) = highlight_with_syntect(code, language) {
            return highlighted;
        }
    }

    let mut output = String::new();
    for line in code.lines() {
        if line.trim().is_empty() {
            output.push('\n');
            continue;
        }

        let colored_line = match language.to_lowercase().as_str() {
            "rust" | "rs" => highlight_rust_line(line),
            "python" | "py" => highlight_python_line(line),
            "javascript" | "js" | "typescript" | "ts" => highlight_js_line(line),
            "json" => highlight_json_line(line),
            "bash" | "sh" | "shell" => highlight_bash_line(line),
            _ => format!("\x1b[37m{}\x1b[0m", line), // Default white
        };

        output.push_str(&colored_line);
        output.push('\n');
    }

    output
}

fn highlight_with_syntect(
    code: &str,
    language: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use syntect::easy::HighlightLines;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::parsing::SyntaxSet;
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let syntax = ss
        .find_syntax_by_extension(language)
        .or_else(|| ss.find_syntax_by_token(language))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, theme);
    let mut output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&escaped);
    }

    Ok(output)
}

fn highlight_rust_line(line: &str) -> String {
    let mut result = String::new();
    let keywords = [
        "fn", "let", "mut", "if", "else", "for", "while", "match", "struct", "enum", "impl", "use",
        "pub",
    ];

    for word in line.split_whitespace() {
        if keywords.contains(&word) {
            result.push_str(&format!("\x1b[94m{}\x1b[0m ", word)); // Blue
        } else if word.starts_with("//") {
            result.push_str(&format!("\x1b[90m{}\x1b[0m ", word)); // Gray
        } else if word.starts_with('"') && word.ends_with('"') {
            result.push_str(&format!("\x1b[92m{}\x1b[0m ", word)); // Green
        } else {
            result.push_str(&format!("\x1b[37m{}\x1b[0m ", word)); // White
        }
    }

    result
}

fn highlight_python_line(line: &str) -> String {
    let mut result = String::new();
    let keywords = [
        "def", "class", "if", "else", "elif", "for", "while", "import", "from", "return", "try",
        "except",
    ];

    for word in line.split_whitespace() {
        if keywords.contains(&word) {
            result.push_str(&format!("\x1b[94m{}\x1b[0m ", word)); // Blue
        } else if word.starts_with("#") {
            result.push_str(&format!("\x1b[90m{}\x1b[0m ", word)); // Gray
        } else if (word.starts_with('"') && word.ends_with('"'))
            || (word.starts_with('\'') && word.ends_with('\''))
        {
            result.push_str(&format!("\x1b[92m{}\x1b[0m ", word)); // Green
        } else {
            result.push_str(&format!("\x1b[37m{}\x1b[0m ", word)); // White
        }
    }

    result
}

fn highlight_js_line(line: &str) -> String {
    let mut result = String::new();
    let keywords = [
        "function", "const", "let", "var", "if", "else", "for", "while", "return", "class",
        "import", "export",
    ];

    for word in line.split_whitespace() {
        if keywords.contains(&word) {
            result.push_str(&format!("\x1b[94m{}\x1b[0m ", word)); // Blue
        } else if word.starts_with("//") {
            result.push_str(&format!("\x1b[90m{}\x1b[0m ", word)); // Gray
        } else if word.starts_with('"') && word.ends_with('"') {
            result.push_str(&format!("\x1b[92m{}\x1b[0m ", word)); // Green
        } else {
            result.push_str(&format!("\x1b[37m{}\x1b[0m ", word)); // White
        }
    }

    result
}

fn highlight_json_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with('"') && trimmed.contains(':') {
        format!("\x1b[96m{}\x1b[0m", line) // Cyan for keys
    } else if trimmed.starts_with('"') {
        format!("\x1b[92m{}\x1b[0m", line) // Green for string values
    } else if trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    {
        format!("\x1b[93m{}\x1b[0m", line) // Yellow for numbers
    } else {
        format!("\x1b[37m{}\x1b[0m", line) // White for everything else
    }
}

fn highlight_bash_line(line: &str) -> String {
    let mut result = String::new();
    let keywords = [
        "if", "then", "else", "fi", "for", "do", "done", "while", "case", "esac", "function",
    ];

    for word in line.split_whitespace() {
        if keywords.contains(&word) {
            result.push_str(&format!("\x1b[94m{}\x1b[0m ", word)); // Blue
        } else if word.starts_with("#") {
            result.push_str(&format!("\x1b[90m{}\x1b[0m ", word)); // Gray
        } else if word.starts_with('$') {
            result.push_str(&format!("\x1b[95m{}\x1b[0m ", word)); // Magenta for variables
        } else {
            result.push_str(&format!("\x1b[37m{}\x1b[0m ", word)); // White
        }
    }

    result
}

pub fn set_model(model_name: &str, config_dir: &Path) -> bool {
    let config_file = config_dir.join("config.json");
    let model = match model_name.to_uppercase().as_str() {
        "GEMINI_1_5_FLASH" => "GEMINI_1_5_FLASH",
        "GEMINI_1_5_PRO" => "GEMINI_1_5_PRO",
        "GEMINI_2_5_FLASH" => "GEMINI_2_5_FLASH",
        "GEMINI_2_5_PRO" => "GEMINI_2_5_PRO",
        _ => {
            println!("Invalid model name. Using default GEMINI_FLASH");
            "GEMINI_FLASH"
        }
    };
    // Write to config file
    let config = serde_json::json!({ "model": model });

    match std::fs::write(config_file, serde_json::to_string_pretty(&config).unwrap()) {
        Ok(_) => {
            println!("Model set to: {}", model);
            true
        }
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
        }
        Err(e) => {
            eprintln!("Failed to clear history: {}", e);
            false
        }
    }
}
pub fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    use std::fs;
    fs::read_to_string(file_path)
}

pub fn sanitize_file_content(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.starts_with("#!"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn display_config(config_dir: &Path) {
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
