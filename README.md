# Todo CLI

A simple command-line note-taking application written in Rust.

## Features

- Add notes quickly from the terminal with a single command
- Automatic timestamp for each entry
- View all notes with their creation times
- Clear all notes when needed
- Persistent storage in JSON format

## Requirements

- Rust (2024 edition or later)

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd todo-cli
cargo build --release
```

The executable will be located at `target/release/todo-cli` (or `todo-cli.exe` on Windows).

### Optional: Add to PATH

For convenient access from anywhere, add the release folder to your system PATH:

**Windows (PowerShell):**
```powershell
$env:PATH += ";C:\path\to\todo-cli\target\release"
```

**Linux/macOS:**
```bash
export PATH=$PATH:/path/to/todo-cli/target/release
```

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

## Data Storage

Notes are stored in a `store.json` file in the project directory. The file is created automatically on the first use.

## License

This project is open source and available under the MIT License.
