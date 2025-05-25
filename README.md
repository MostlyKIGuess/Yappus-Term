# Yappus Terminal

A terminal interface for your AI terminal assistant, warning it YAPS! xD.

Local options coming soon using ollama.

## Features

- Interactive command-line interface for chatting with Gemini AI
- Support for multiple Gemini models including latest Gemini 2.0 Flash
- Persistent chat history saved between sessions
- Command-line arguments for direct queries and configuration
- Interactive commands within the chat interface
- File integration with `/file` command for context-aware discussions
- Directory navigation with built-in file system commands (`/ls`, `/cd`, `/pwd`)
- Command piping to combine shell commands with AI queries
- Export chat history to JSON files
- Configuration management for API keys and model selection
- Syntax highlighting for code blocks in responses
- Tab completion and command history support
- Context awareness that maintains conversation flow

## In Works
- [ ] Using history for more context.
- [x] Using current directory and .git for more knowledge.
- [ ]Using history to predict next questions and help.
- [ ]Fully local mode using Ollama
- [~(kinda lol) ]RAG support with document indexing
- [ ] CMD to include the actual shell commands for better interactive.

## Prerequisites

- Rust and Cargo (latest stable version)
- A Google Gemini API key (obtain from https://aistudio.google.com/app/apikey)

## Installation

### For Debian/Ubuntu-based systems:

You can install Yappus directly from our APT repository:

```bash
# Download the installation script
curl -O https://raw.githubusercontent.com/MostlyKIGuess/Yappus-Term/main/install-yappus.sh

chmod +x install-yappus.sh

# runnn
./install-yappus.sh
```
### For Arch Linux:

Install from the AUR:
```sh
yay -S yappus
```

Or manually using the PKGBUILD:

```sh
git clone https://github.com/MostlyKIGuess/Yappus-Term.git
cd Yappus-Term
makepkg -si
```

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/MostlyKIGuess/Yappus-Term.git
   cd yappus-term
   ```

2. Run the application and it will prompt you for an API key during first-time setup.

## Building

Build the application using Cargo:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/yappus`.

## Installing in your Shell

You can install the application in your PATH using Cargo:

```bash
cargo install --path .
```

## Running

You can run the application directly using Cargo:

```bash
cargo run
```

Or execute the binary after building:

```bash
./target/release/yappus
```

Or if installed in your PATH:

```bash
yappus
```

## Configuration

Yappus stores its configuration and history files in:
- Linux/macOS: `~/.config/yappus-term/`
- Windows: `%APPDATA%\yappus\yappus-term\config\`

The following files are created:
- `api_key` - Stores your Gemini API key
- `config.json` - Stores your preferred model configuration
- `chat_history.json` - Stores your chat history
- `readline_history.txt` - Stores command history

## Usage

### Command Line Arguments

```bash
# ask a question directly, you can use or not use the double quotes
yappus "How do I find large files in Linux?"

# model change, type just yappus model to see the list
yappus model GEMINI_2_5_PRO

# shows chat history
yappus history

#clears  history
yappus clear-history

# run the setup again
yappus setup

# version info
yappus version

# current config display
yappus config

# reset or check the api key
yappus key
yappus key --reset

# analyze a file with optional query
yappus file script.sh "explain this script"

# if you wanna export your chats
yappus export ~/my_chat_export.json
```

### Interactive Mode

Start interactive mode by running `yappus` without arguments:

- Type your message at the prompt (`>`) and press Enter
- The AI will respond with its message
- Type `exit` to quit the application

#### Available Interactive Commands:

- `/help` - Show help message with available commands
- `/model [name]` - View or change the Gemini model
- `/history` - View your chat history
- `/clearhistory` - Clear your chat history
- `/setup` - Run the setup process again
- `/version` - Show version information
- `/config` - Display current configuration
- `/key [reset]` - Check or reset API key
- `/export [path]` - Export chat history to file
- `/clear` - Clear the terminal screen
- `/file <path> [query]` - Include file content in conversation
- `/ls [path]` - List directory contents
- `/cd <path>` - Change directory
- `/pwd` - Show current directory
- `exit` - Exit the application

#### Advanced Features

**Command Piping**: Combine shell commands with AI queries
```bash
> /ls | what programming languages are used in this project?
> /pwd | what should I do in this directory?
```

**File Context Analysis**: 
```bash
> /file package.json
> What dependencies does this project use?
> How can I optimize this configuration?
```

## Available Models

- `GEMINI_FLASH` (2.0) - Default model, latest and greatest
- `GEMINI_2_5_PRO` - Most capable model for complex tasks
- `GEMINI_2_5_FLASH` - High performance with excellent reasoning
- `GEMINI_1_5_PRO` - Powerful legacy model
- `GEMINI_1_5_FLASH` - Fast and efficient

Switch models anytime:
```bash
yappus model GEMINI_2_5_PRO  # CLI
/model GEMINI_2_5_PRO        # Interactive
```

### APT Building
```sh
docker run --rm -v $(pwd):/build -w /build debian:bookworm bash -c "apt-get update && apt-get install -y build-essential debhelper curl pkg-config libssl-dev && curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && export PATH=\$HOME/.cargo/bin:\$PATH && dpkg-buildpackage -us -uc -b -d && cp -v /*.deb /build/"
```

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
