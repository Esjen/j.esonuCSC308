use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        println!("\n======== Notes Menu ========");
        println!("1. Add a note");
        println!("2. View all notes");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}

fn add_note() {
    print!("Enter your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();
    let note = note.trim();

    if note.is_empty() {
        println!("Note cannot be empty.");
        return;
    }

    // Simple timestamp (Unix seconds)
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let formatted = format!("[{}] {}\n", timestamp, note);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(formatted.as_bytes()).unwrap();

    println!("Note added.");
}

fn view_notes() {
    match read_to_string("notes.txt") {
        Ok(contents) => {
            if contents.trim().is_empty() {
                println!("No notes yet.");
            } else {
                println!("\n===== All Notes =====");
                print!("{}", contents);
            }
        }
        Err(_) => println!("No notes file found."),
    }
}
