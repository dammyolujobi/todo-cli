# Todo CLI

A simple command-line note-taking application written in Rust.

## Features

- Add notes quickly from the terminal with a single command
- Automatic timestamp for each entry
- View all notes with their creation times
- Clear all notes when needed
- Persistent storage in a cross-platform config directory

## Requirements

- Rust (2024 edition or later)

## Installation

Clone the repository:

```bash
git clone https://github.com/dammyolujobi/todo-cli.git
cd todo-cli
```

### Install globally (Recommended)

Install the CLI tool to Cargo's bin directory (automatically added to PATH):

```bash
cargo install --path .
```

After installation, you can use `todo-cli` from anywhere on your system.

To update after pulling new changes:
```bash
git pull
cargo install --path .
```

To uninstall:
```bash
cargo uninstall todo-cli
```

### Or build locally

If you prefer to build without installing:

```bash
cargo build --release
```

The executable will be located at `target/release/todo-cli` (or `todo-cli.exe` on Windows)

## Usage

The application provides three subcommands:

### Add a note

```bash
todo-cli add "Your note text"
```

Or using cargo:
```bash
cargo run -- add "Your note text"
```

### List all notes

```bash
todo-cli list
```

### Clear all notes

```bash
todo-cli clear
```

### Example Session

```bash
# Add some notes
$ todo-cli add "Buy groceries"
Node Added!

$ todo-cli add "Call dentist"
Node Added!

# View all notes
$ todo-cli list
Note                 Time
----------------------------------------
Buy groceries        14:23:45 PM
Call dentist         14:24:12 PM

# Clear all notes
$ todo-cli clear
List Cleared
```

## Dependencies

- `chrono` - Date and time handling
- `serde` - Serializing and Deserializing Data Structures
- `serde_json` - Mapping JSON to Rust Data Structures
- `clap` - Command-line argument parser
- `dirs` - Cross-platform system directories

## Data Storage

Notes are stored in a `store.json` file in your system's config directory:

- **Linux**: `~/.config/todo-cli/store.json`
- **macOS**: `~/Library/Application Support/todo-cli/store.json`
- **Windows**: `%APPDATA%\todo-cli\store.json` (typically `C:\Users\USERNAME\AppData\Roaming\todo-cli\store.json`)

The directory and file are created automatically on the first use.

## License

This project is open source and available under the MIT License.