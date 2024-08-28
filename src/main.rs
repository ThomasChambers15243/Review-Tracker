use std::{collections::HashMap, process, io::{self, Write}};

// Crates
use dialoguer::{theme::ColorfulTheme, Input, Select};

// Mods
pub mod storage;
pub mod tracker;
use storage::Note;
use thiserror::Error;
use tracker::{load_map, view_map, ASCII};

// Choice menus
const YES_NO_CHOICES: &'static [&str;2] = &["YES", "NO"];

const MAIN_MENU_CHOICES: &'static [&str;5] = &[
    "Add Note",
    "Remove Note",
    "View Notes",
    "Generate Review",
    "Quit"
    ];

static mut CLEAR: bool = false;

#[derive(Error, Debug)]
enum main_error {
    #[error("{0}")]
    driver_error(String)
}
    
fn main() {
    // Enable screen clearing
    //let mut CLEAR: bool = false;
    let clear_choice = Select::new()
        .with_prompt(format!("Enable screen clearning\n{}Warning{} - Wipes current terminal",ASCII["RED"],ASCII["RESET"]))
        .items(YES_NO_CHOICES)
        .default(0)
        .interact()
        .unwrap();
    unsafe {
        match YES_NO_CHOICES[clear_choice] {
            "YES" => CLEAR = true,
            _ => CLEAR = false,
    }
}
    // Load in notes
    println!("Loading...");
    let mut note_map: HashMap<String, Note> = HashMap::new();
    match load_map() {
        Ok(m) => note_map = m,
        Err(e) => {
            println!("Error: Could not load note data, check config file and json file.
                    \nError {e}
                    \nEnding Process...");
            process::exit(1);
        },
    };
    
    // Main Loop
    loop {
        let menu_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Main Menu")
        .items(MAIN_MENU_CHOICES)
        .default(0)
        .interact()
        .unwrap();
    
        // Reset Screen after selection, leaving the result message from the last
        // message on screen for the user
        clear_screen();
        match MAIN_MENU_CHOICES[menu_choice] {
            "Add Note" => {                
                handle_map_operation(&mut note_map, |m| io_add_note(m));
            },
            "Remove Note" => {
                handle_map_operation(&mut note_map, |m| io_remove_note(m));
            }
            "View Notes" => 
                view_map(&note_map),
            "Generate Review" => {
                handle_map_operation(&mut note_map, |m| io_generate_review(m));
            },
            "Quit" => {            
                clear_screen(); // Clear screen and reset cursor before exiting
                process::exit(0);
            },
            _ => {
                println!("Something went wrong, ending process");            
                clear_screen();
                process::exit(0);
            },
        };
    }
}



fn clear_screen() {
    // Restore cursor to the saved position and clear everything below
    unsafe {
        if CLEAR {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush().unwrap();
        }
    }
}

// Handles and formats the result messages from main loop operations
// Keeps main loop cleaner
fn handle_map_operation<F>(note_map: &mut HashMap<String, Note>, operation: F) 
where 
    F: Fn(&mut HashMap<String, Note>) -> Result<String, main_error>
{ 
    match operation(note_map) {
        Ok(message) => {
            println!("{}{}{}", ASCII["GREEN"], message, ASCII["RESET"]);
        },                    
        Err(e) => println!("{} {} {}",ASCII["RED"], e, ASCII["RESET"]),
    };
}


// Gets and adds a new Note to the note name
// Requests a name from the user, validates the name
// and creates as new Note, adding it to the map.
fn io_add_note(note_map: &mut HashMap<String, Note>) -> Result<String, main_error> {            

    // Notes values
    let freq: u16 = 0;
    let last_accessed = "Today".to_string();    
    let name: String = Input::new()
        .with_prompt("Enter the New Notes Name")
        .validate_with(|input: &String| -> Result<(), &str> {
            // Check Note of same is not already in map
            match note_map.contains_key(&input.to_lowercase()) {
                false => Ok(()),
                true => Err("Note with same name already added\nTry Again"),
            }
        })
        .interact()
        .unwrap();

    // Gives user an out incase they're filled with a deep regret over
    // their note choice
    let sure = Select::new()
        .with_prompt("Add Note?")
        .items(YES_NO_CHOICES)
        .default(0)
        .interact()
        .unwrap();

    match YES_NO_CHOICES[sure] {
        "YES" => {
            note_map.insert(name.clone(), Note::new(name, freq, last_accessed));
            Ok("Success! Note Added".to_string())    
        },
        _ => Err(main_error::driver_error("No Note was added".to_string()))
    }
}


// Gets and removes a Note from the map
fn io_remove_note(note_map: &mut HashMap<String, Note>) -> Result<String, main_error> {
    let name: String = Input::new()
        .with_prompt("Enter Notes Name to be Removed")
        .interact()
        .unwrap();
    
    // Gives user an out incase they're filled with a deep regret over
    // their note choice
    let sure = Select::new()
        .with_prompt("Remove Note?")
        .items(YES_NO_CHOICES)
        .default(0)
        .interact()
        .unwrap();

    // Gives user an out incase they're filled with a deep regret over
    // their note choice
    match YES_NO_CHOICES[sure] {
        "YES" => {
            // Removes note from map
            if let Some((k,v)) = note_map.remove_entry(&name) {
                Ok(format!("{k} was remove with values:\nFreq: {}\nLast Accessed:{}", v.freq, v.last_accessed))
            } else {
                Err(main_error::driver_error("Could not find note to remove".to_string()))
            }
        },
        _ => Err(main_error::driver_error("No Note was added".to_string()))
    }
}


fn io_generate_review(note_map: &mut HashMap<String, Note>) -> Result<String, main_error> {
    Ok("WOOP".to_string())
}