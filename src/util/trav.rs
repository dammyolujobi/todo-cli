use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{BufWriter};
use std::io::{BufReader};
use chrono::Utc;

#[derive(Serialize, Deserialize,Debug)]
struct Notes{
    index: i32,
    note: String,
    time: String,
}



#[derive(Serialize,Deserialize,Debug)]
struct CompletedNotes {
    index:i32,
    note:String,
    time:String,
    date:String
}
fn increase_comp_index(todos:&Vec<CompletedNotes>) -> i32 {
    return todos.last().map_or(1, |t| t.index + 1);
}

pub fn delete_completed(index:usize) {
        let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("completed.json");

    if Path::exists(&store_path) {
        let store = File::open(&store_path).unwrap();
        let reader = BufReader::new(store);

        let mut todos:Vec<Notes> = serde_json::from_reader(reader).unwrap();

        if index > todos.len() {
            println!("No note with that index")
        }
        else {
            todos.swap_remove(index-1);
        
        for i in 0..todos.len(){
            if i >index {
                todos[i].index = i as i32 -1
            }
        }


        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();
        println!("Value Deleted");
        }
        
    }
}
pub fn delete(index:usize) {
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");

    if Path::exists(&store_path) {
        let store = File::open(&store_path).unwrap();
        let reader = BufReader::new(store);

        let mut todos:Vec<Notes> = serde_json::from_reader(reader).unwrap();

        if index > todos.len() {
            println!("No note with that index")
        }
        else {
            todos.swap_remove(index-1);
        
        for i in 0..todos.len(){
            if i >index {
                todos[i].index = i as i32 -1
            }
        }


        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();
        println!("Value Deleted");
        }
        
    }
}

pub fn complete (index:usize) {
    let config_dir = dirs::config_dir().expect("Could not find config dir");
    let store_path = config_dir.join("store.json");
    let utc_now = Utc::now();
    let date  = utc_now.format("%A %d %Y").to_string();


    if Path::exists(&store_path) {
        let store = File::open(&store_path).unwrap();
        let reader = BufReader::new(store);

        let mut todos:Vec<Notes> = serde_json::from_reader(reader).unwrap();

        let temp = todos.remove(index-1);

        for i in 0..todos.len(){
            if i > index {
                todos[i].index = i as i32 -1
            }
        }

        let file = File::create(&store_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos).unwrap();
        

        let completed_path = config_dir.join("completed.json");

        if Path::new(&completed_path).exists() {
            let file = File::open(&completed_path).unwrap();
            let reader = BufReader::new(file);


            let mut comp: Vec<CompletedNotes> = serde_json::from_reader(reader).unwrap_or_default();
             
            let compn = CompletedNotes{
                index : increase_comp_index(&comp),
                note : temp.note,
                time: temp.time,
                date:date
                
            };

            comp.push(compn);

            let file = File::create(&completed_path).unwrap();
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &comp).unwrap();
        }
        else {
            File::create_new(&completed_path).unwrap();

            let file = File::open(&completed_path).unwrap();
            let writer = BufWriter::new(file);

            serde_json::to_writer_pretty(writer, &CompletedNotes{
                index : temp.index,
                note : temp.note,
                time: temp.time,
                date:date
                
            }).unwrap();
        }
    }

}


pub fn check_completed(){
    let config_path = dirs::config_dir().expect("File not found");
    let completed_path = config_path.join("completed.json");

    if !Path::new(&completed_path).exists() {
        File::create(&completed_path).unwrap();
    }

    let store = File::open(&completed_path).unwrap();

    if store.metadata().unwrap().len() == 0 {
        println!("{:<20} {:<20} {:<20}","Index","Note","Time Completed");
        println!("{:-<62}","");
    }   
    
    else {
        let reader = BufReader::new(&store);

        let notes: Vec<CompletedNotes> = serde_json::from_reader(reader).unwrap();

        println!("{:<20} {:<20} {:<20}","Index","Note","Time Completed");
        println!("{:-<62}","");

        for note in notes {
            println!("{:<20} {:<20} {:<20}",note.index,note.note,note.date)
            
        }
    }
}