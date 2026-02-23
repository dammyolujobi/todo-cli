use chrono::Utc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader};
use std::path::Path;
use std::vec;
mod util;
use util::Cli;
use util::Commands;
use dirs;

#[derive(Serialize, Deserialize)]
struct Data {
    note: String,
    time: String,
}



fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { note }=> {
            add_new_entry(note);
            println!("Node Added!");
        }
        Commands::List {  }=> {
            get_todo();
        }
        Commands::Clear {  }=>{
            clear();
            println!("List Cleared")
        }
    }
}

fn add_new_entry(data: String) {
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");

    let utc_now = Utc::now();
    let utc_now_string = utc_now.format("%H:%M:%S %p").to_string();

    let note = Data {
        note: data,
        time: utc_now_string,
    };

    if Path::new(&store_path).exists() {
        let file = File::open(&store_path).unwrap();
        let reader = BufReader::new(file);
        let mut todos: Vec<Data> = serde_json::from_reader(reader).unwrap();

        todos.push(note);

        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();
    } else {
        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        let todos: Vec<Data> = vec![note];
        serde_json::to_writer_pretty(writer, &todos).unwrap();
    }
}

fn get_todo() {
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");

    if !Path::new(&store_path).exists()  {
        let _ = File::create(&store_path).unwrap();
    }

    let store = File::open(&store_path).unwrap();
    
    if store.metadata().unwrap().len() == 0 {
        println!("{:<20} {:<20}", "Note", "Time");
        println!("{:-<40}", "");
    }
    else {
        println!("{:<20} {:<20}", "Note", "Time");
        println!("{:-<40}", "");

        let reader = BufReader::new(store);
        let todos: Vec<Data> = serde_json::from_reader(reader).unwrap();

        for todo in todos {
            println!("{:<20} {:<20}", todo.note, todo.time);
        }
    }
    
}

fn clear(){
    let config_dir = dirs::config_dir().expect("Could not find config dir").join("store.json");
    let store_path = config_dir.join("store.json");

    File::create(&store_path).unwrap();
}