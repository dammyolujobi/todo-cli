use chrono::Utc;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();

    println!("Enter value to notes or quit to close the program");

    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read message");

        let new_text = input_text.trim().to_string();

        if new_text.to_lowercase() != "quit" {
            add_new_entry(new_text, &mut store);
        } else if new_text.to_lowercase() == "quit" {
            println!("\n");
            break;
        }
    }
    get_todo(&mut store);
}

fn add_new_entry(data: String, store: &mut HashMap<String, String>) {
    let utc_now = Utc::now();

    let utc_now_string = utc_now.format("%H:%M:%S %p").to_string();
    let _ = &store.insert(data, utc_now_string);
}

fn get_todo(store: &mut HashMap<String, String>) {
    println!("{:<20} {:<20}", "Note", "Time");
    println!("{:-<40}", "");
    for (key, value) in store.iter() {
        println!("{:<20} {:<20}", key, value);
    }
}
