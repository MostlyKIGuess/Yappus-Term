# Yappus Terminal

A simple terminal-based chat application that uses Google's Gemini API to provide AI-powered responses.

## Features

- Interactive command-line interface for chatting with Gemini AI
- Persistent chat history saved between sessions
- Simple and easy-to-use interface


### Planned Features:
- Reading error logs , understanding curr directory strucutre.
- Different endpoints to clear/view change history.
- Modify Models, Guide to API gen.

## Prerequisites

- Rust and Cargo (latest stable version)
- A Google Gemini API key

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/yappus-term.git
   cd yappus-term
   ```

2. Create a `.env` file by copying the example:
   ```bash
   cp .env.example .env
   ```

3. Add your Gemini API key to the `.env` file:
   ```
   GEMINI_API_KEY=your_api_key_here
   ```

## Building

Build the application using Cargo:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/yappus-term`.

## Installing in your Shell

You can install the application in your shell by moving the binary to a directory in your PATH:

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
./target/release/yappus-term
```




## Usage


### If Running the Binary
- Type your message at the prompt (`>`) and press Enter
- The AI will respond with its message
- Type `exit` to quit the application
- Chat history is automatically saved to `chat_history.json` in the project root

### If installed via Shell
- You can write 1`1

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
