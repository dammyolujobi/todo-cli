use chrono::Utc;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter value to notes or quit to close the program");

    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read message");

        if input_text.trim().to_string().to_lowercase() != "quit" {
            add_new_entry(input_text.trim().to_string());
        } else if input_text.trim().to_string().to_lowercase() == "quit" {
            break;
        }
    }
}

fn add_new_entry(data: String) {
    let mut map: HashMap<String, String> = HashMap::new();
    let utc_now = Utc::now();

    let utc_now_string = utc_now.time().to_string();
    map.insert(data, utc_now_string);

    for (dt, time) in &map {
        println!("{} {} ", dt, time);
    }
}
