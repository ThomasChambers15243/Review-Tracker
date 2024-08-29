use std::{collections::HashMap, process, io::{self, Write}};

// Crates
use dialoguer::{theme::ColorfulTheme, Input, Select};

// Mods
pub mod storage;
pub mod tracker;
use storage::Note;
use thiserror::Error;
use tracker::{find_least_common_notes, load_map, save_map, update_reviewed_notes, view_map, ASCII};

// Choice menus
const YES_NO_CHOICES: &'static [&str;2] = &["YES", "NO"];

const MAIN_MENU_CHOICES: &'static [&str;5] = &[
    "Add Note",
    "Remove Note",
    "View Notes",
    "Generate Review",
    "Quit"
    ];

// Boolean flag to indicate whether the user want to clear the screen after inputs or not
static mut CLEAR: bool = false;

#[derive(Error, Debug)]
enum MainError {
    #[error("{0}")]
    DriverError(String)
}
    
fn main() {
    // Loads in notes
    println!("Loading...");
    let mut note_map: HashMap<String, Note>;
    match load_map() {
        Ok(map) => note_map = map,
        Err(e) => {
            println!("Could not load map due to error {}\nEnding Process...",e);
            process::exit(1);
        },
    }

    // Enable screen clearing
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

    // Main Loop
    clear_screen();
    loop {
        match sync_map(note_map) {
            Ok(map) => note_map = map,
            Err(e) => {
                println!("Could not save and sync the map due to error{}\nEnding Process...",e);
                process::exit(1);
            },
        }

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
            "View Notes" => {
                // Handle case where map is empty
                match io_handle_empty_map(&note_map) {
                    Ok(_) => view_map(&note_map),
                    Err(message) => println!("{}", message),
                }
            },
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

fn sync_map(note_map: HashMap<String, Note>) -> Result<HashMap<String, Note>, MainError>{
    save_map(note_map);
    match load_map() {
        Ok(m) => Ok(m),
        Err(e) => Err(MainError::DriverError(format!("Error: Could not load note data, check config file and json file.
                    \nError {e}"
        ))),
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
    F: Fn(&mut HashMap<String, Note>) -> Result<String, MainError>
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
fn io_add_note(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {            
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
        _ => Err(MainError::DriverError("No Note was added".to_string()))
    }
}


// Gets and removes a Note from the map
fn io_remove_note(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    // Handle case where map is empty
    io_handle_empty_map(note_map)?;
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
            if let Some(note_name) = note_map.keys()
                .find(|key| key.to_lowercase() == name.to_lowercase()).cloned()
            {
                if let Some(note) = note_map.remove(&note_name) {
                    return Ok(format!(
                        "{} was removed with values:\nFreq: {}\nLast Accessed: {}",
                        note.name, note.freq, note.last_accessed
                    ));
                } else {
                    return Err(MainError::DriverError("Could not find note to remove".to_string()));
                }
            }
            Err(MainError::DriverError("Note not found".to_string()))
        },
        _ => Err(MainError::DriverError("No Note was removed".to_string())),
    }
    
}


fn io_generate_review(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    // Handle case where map is empty
    io_handle_empty_map(note_map)?;

    // Start review
    let reviewed: Vec<Note> = find_least_common_notes(note_map, 3);
    println!("Notes to Review:\n");
    for note in &reviewed {
        println!("{}{}{}",ASCII["BLUE"], note.name, ASCII["BLUE"])
    }
    // Save Review
    let save = Select::new()
        .with_prompt("Save Review?")
        .items(YES_NO_CHOICES)
        .default(0)
        .interact()
        .unwrap();

    match YES_NO_CHOICES[save] {
        "YES" => {
            update_reviewed_notes(note_map, reviewed);
            Ok("Notes Saved".to_string())
        },
        _ => Err(MainError::DriverError("Notes were not saved".to_string()))
    }
}

fn io_handle_empty_map(note_map: &HashMap<String, Note>) -> Result<String, MainError> {
    // Check map is empty
    match note_map.is_empty() {
        true => Err(MainError::DriverError(format!(
                "{}No notes to review\nTry adding some notes with {}Add Note{}",
                ASCII["GREEN"], ASCII["BOLD"], ASCII["RESET"]).to_string())),
        false => Ok("".to_string()),
    }
}