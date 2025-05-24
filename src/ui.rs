use colored::*;

pub struct HelpFormatter;

impl HelpFormatter {
    pub fn display_full_help() {
        println!();
        println!(
            "{}",
            "=========================================".bright_cyan()
        );
        println!(
            "{}",
            "              HELP MENU                  ".bright_cyan()
        );
        println!(
            "{}",
            "=========================================".bright_cyan()
        );
        println!();

        println!("{}", "USAGE:".bright_yellow().bold());
        println!(
            "  {} [QUERY]                 Ask a question directly",
            "yappus".green()
        );
        println!(
            "  {} [COMMAND] [OPTIONS]     Run a specific command",
            "yappus".green()
        );
        println!();

        println!("{}", "CLI COMMANDS:".bright_yellow().bold());
        Self::print_command("model [NAME]", "View or change the Gemini model");
        Self::print_command("history", "View your chat history");
        Self::print_command("clear-history", "Clear your chat history");
        Self::print_command("setup", "Run the setup process again");
        Self::print_command("version", "Show version information");
        Self::print_command("config", "Display current configuration");
        Self::print_command("key [--reset]", "Check or reset API key");
        Self::print_command("export [PATH]", "Export chat history to file");
        Self::print_command("file PATH [QUERY]", "Include file content in your query");
        Self::print_command("help", "Show this help message");
        println!();

        println!("{}", "INTERACTIVE COMMANDS:".bright_yellow().bold());
        Self::print_interactive_command("/help", "Show help information");
        Self::print_interactive_command("/model [name]", "View or change the Gemini model");
        Self::print_interactive_command("/history", "View your chat history");
        Self::print_interactive_command("/ls [path]", "List files in current/specified directory");
        Self::print_interactive_command("/file <path> [q]", "Include file content in your query");
        Self::print_interactive_command("/clearhistory", "Clear your chat history");
        Self::print_interactive_command("/setup", "Run the setup process again");
        Self::print_interactive_command("/version", "Show version information");
        Self::print_interactive_command("/config", "Display current configuration");
        Self::print_interactive_command("/key [reset]", "Check or reset API key");
        Self::print_interactive_command("/export [path]", "Export chat history to file");
        Self::print_interactive_command("/clear", "Clear the terminal screen");
        Self::print_interactive_command("/pwd", "Print working directory");
        Self::print_interactive_command("/cd <path>", "Change directory");
        Self::print_interactive_command("exit", "Exit the application");
        println!();

        Self::display_available_models();
        Self::display_examples();

        println!("{}", "PRO TIPS:".bright_yellow().bold());
        println!(
            "  * Press {} for auto-completion",
            "TAB".bright_green().bold()
        );
        println!(
            "  * Use {} and {} to navigate history",
            "UP".bright_blue(),
            "DOWN".bright_blue()
        );
        println!("  * Type {} to clear screen", "/clear".cyan());
        println!("  * Use {} for context-aware queries", "/file".cyan());
        println!();
    }

    pub fn display_available_models() {
        println!("{}", "AVAILABLE MODELS:".bright_yellow().bold());
        Self::print_model("GEMINI_FLASH", "Latest Gemini 2.0 model (default)");
        Self::print_model("GEMINI_2_5_PRO", "Most powerful for complex tasks");
        Self::print_model(
            "GEMINI_2_5_FLASH",
            "High performance with excellent reasoning",
        );
        Self::print_model("GEMINI_1_5_PRO", "Very capable legacy model");
        Self::print_model("GEMINI_1_5_FLASH", "Fast and efficient responses");
        println!();
    }

    pub fn display_examples() {
        println!("{}", "EXAMPLES:".bright_yellow().bold());
        Self::print_example(
            "yappus \"How do I find large files in Linux?\"",
            "Ask a direct question",
        );
        Self::print_example(
            "yappus model GEMINI_2_5_PRO",
            "Change to the most powerful model",
        );
        Self::print_example(
            "yappus file script.sh \"explain this script\"",
            "Analyze a file",
        );
        Self::print_example("yappus history", "View your chat history");
        Self::print_example(
            "yappus export ~/my_backup.json",
            "Export your conversations",
        );
        println!();

        println!("{}", "Interactive mode examples:".bright_cyan());
        Self::print_example("> How do I backup my home directory?", "");
        Self::print_example("> /model GEMINI_1_5_PRO", "");
        Self::print_example("> /file ~/.bashrc explain this config", "");
        Self::print_example("> /ls ~/Documents", "");
        Self::print_example("> /export ~/yappus_backup.json", "");
        println!();
    }

    fn print_command(cmd: &str, desc: &str) {
        println!("  {:30} {}", cmd.green().bold(), desc.white());
    }

    fn print_interactive_command(cmd: &str, desc: &str) {
        println!("  {:30} {}", cmd.cyan().bold(), desc.white());
    }

    fn print_model(name: &str, desc: &str) {
        println!("  {:25} {}", name.bright_green().bold(), desc.white());
    }

    fn print_example(example: &str, desc: &str) {
        if desc.is_empty() {
            println!("  {}", example.bright_white());
        } else {
            println!("  {:50} {}", example.bright_white(), desc.bright_black());
        }
    }
}

