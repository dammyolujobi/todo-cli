use chrono::Utc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader};
use std::path::Path;
use std::{usize, vec};
mod util;
use util::Cli;
use util::Commands;
use dirs;
use util::trav::delete;
use util::trav::complete;
use util::trav::check_completed;
use crate::util::increase_index;
use crate::util::trav::delete_completed;



#[derive(Serialize, Deserialize,Debug)]
struct Notes {
    index: i32,
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
        Commands::Delete{index}=>{
            delete(index);
        }
        Commands::Complete { index }=> {
            complete(index);
            println!("Task Completed")
        }
        Commands::GetCompleted {  }=> {
            check_completed();
        }
        Commands::DeleteCompleted{ index}=> {
            delete_completed(index);
        }
    }
}

fn add_new_entry(data: String) {
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");

    let utc_now = Utc::now();
    let utc_now_string = utc_now.format("%H:%M:%S %p").to_string();

    if Path::new(&store_path).exists() {
        let file = File::open(&store_path).unwrap();
        let reader = BufReader::new(file);
        let mut todos: Vec<Notes> = serde_json::from_reader(reader).unwrap_or_default();

        let note = Notes {
            index: increase_index(&todos),
            note:data,
            time: utc_now_string,
        };

        todos.push(note);
    
        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();

    } else {
        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        let note = Notes{
            index:1,
            note:data,
            time:utc_now_string
        };

        let todos: Vec<Notes> = vec![note];
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
        println!("{:<20} {:<20} {:<20}","Index","Note", "Time");
        println!("{:-<62}","");
    }
    else {
        println!("{:<20} {:<20} {:<20}","Index","Note", "Time");
        println!("{:-<62}", "");

        let reader = BufReader::new(store);
        let todos: Vec<Notes> = serde_json::from_reader(reader).unwrap();

        for todo in todos {
            println!("{:<20} {:<20} {:<20}",todo.index, todo.note, todo.time);
        }
    }
    
}

fn clear(){
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");

    File::create(store_path).unwrap();
    
}
