use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufReader};
use std::path::Path;
use std::vec;

#[derive(Serialize, Deserialize)]
struct Data {
    note: String,
    time: String,
}

fn main() {
    println!("Enter value to notes or quit to close the program");

    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read message");

        let new_text = input_text.trim().to_string();

        if new_text.to_lowercase() != "quit" {
            add_new_entry(new_text);
        } else if new_text.to_lowercase() == "quit" {
            println!("\n");
            break;
        }
    }
    get_todo();
}

fn add_new_entry(data: String) {
    let utc_now = Utc::now();
    let utc_now_string = utc_now.format("%H:%M:%S %p").to_string();

    let note = Data {
        note: data,
        time: utc_now_string,
    };

    if Path::new("store.json").exists() {
        let file = File::open("store.json").unwrap();
        let reader = BufReader::new(file);
        let mut todos: Vec<Data> = serde_json::from_reader(reader).unwrap();

        todos.push(note);

        let file = File::create("store.json").unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();
    } else {
        let file = File::create("store.json").unwrap();
        let writer = BufWriter::new(file);
        let todos: Vec<Data> = vec![note];
        serde_json::to_writer_pretty(writer, &todos).unwrap();
    }
}

fn get_todo() {
    println!("{:<20} {:<20}", "Note", "Time");
    println!("{:-<40}", "");
    let store = File::open("store.json").unwrap();
    let reader = BufReader::new(store);

    let todos: Vec<Data> = serde_json::from_reader(reader).unwrap();

    for todo in todos {
        println!("{:<20} {:<20}", todo.note, todo.time);
    }
}
