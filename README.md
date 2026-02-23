# Todo CLI

A simple command-line todo/task management application written in Rust.

## Features

- Add tasks quickly from the terminal with a single command
- Automatic timestamp and index for each task
- View all tasks with their creation times and indices
- Delete specific tasks by index
- Mark tasks as complete (moves them to a completed tasks list)
- View all completed tasks with completion dates
- Clear all tasks when needed
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

The application provides six subcommands:

### Add a task

```bash
todo-cli add "Your task description"
```

Or using cargo:
```bash
cargo run -- add "Your task description"
```

### List all tasks

```bash
todo-cli list
```

### Delete a task

Delete a specific task by its index number:

```bash
todo-cli delete <index>
```

### Complete a task

Mark a task as completed (removes it from the active list and adds it to completed tasks):

```bash
todo-cli complete <index>
```

### View completed tasks

```bash
todo-cli get-completed
```

### Clear all tasks

```bash
todo-cli clear
```

### Example Session

```bash
# Add some tasks
$ todo-cli add "Buy groceries"
Node Added!

$ todo-cli add "Call dentist"
Node Added!

$ todo-cli add "Finish project report"
Node Added!

# View all tasks
$ todo-cli list
Index                Note                 Time                
--------------------------------------------------------------
1                    Buy groceries        14:23:45 PM         
2                    Call dentist         14:24:12 PM         
3                    Finish project report 14:25:03 PM         

# Complete a task
$ todo-cli complete 2
Task Completed

# Delete a task
$ todo-cli delete 1
Value Deleted

# View remaining tasks
$ todo-cli list
Index                Note                 Time                
--------------------------------------------------------------
1                    Finish project report 14:25:03 PM         

# View completed tasks
$ todo-cli get-completed
Index                Note                 Time Completed      
--------------------------------------------------------------
1                    Call dentist         Sunday 23 2026

# Clear all tasks
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

Tasks are stored in JSON files in your system's config directory:

### Active Tasks
`store.json` contains all active (incomplete) tasks:

- **Linux**: `~/.config/store.json`
- **macOS**: `~/Library/Application Support/store.json`
- **Windows**: `%APPDATA%\store.json` (typically `C:\Users\USERNAME\AppData\Roaming\store.json`)

### Completed Tasks
`completed.json` contains all completed tasks:

- **Linux**: `~/.config/completed.json`
- **macOS**: `~/Library/Application Support/completed.json`
- **Windows**: `%APPDATA%\completed.json` (typically `C:\Users\USERNAME\AppData\Roaming\completed.json`)

Both files are created automatically on first use.

## License

This project is open source and available under the MIT License.