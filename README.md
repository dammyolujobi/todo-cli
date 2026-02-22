# Todo CLI

A simple command-line note-taking application written in Rust.

## Features

- Add notes quickly from the terminal
- Automatic timestamp for each entry
- View all notes with their creation times

## Requirements

- Rust (2024 edition or later)

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd todo-cli
cargo build --release
```

## Usage

Run the application:

```bash
cargo run
```

Enter your notes one per line. Type `quit` to exit and view all entries with timestamps.

### Example

```
Enter value to notes or quit to close the program
Buy groceries
Call dentist
quit

Note                 Time
----------------------------------------
Buy groceries        14:23:45 PM
Call dentist         14:24:12 PM
```

## Dependencies

- `chrono` - Date and time handling

## License

This project is open source and available under the MIT License.
