use std::{collections::HashMap, error::Error, io::{self, Write}};
use std::{thread, time::Duration, process};


// Crates
use dialoguer::{theme::ColorfulTheme, Input, InputValidator, Select};
use lazy_static::lazy_static;

// Mods
pub mod storage;
pub mod tracker;
use storage::Note;
use thiserror::Error;
use tracker::load_map;

// User flag for weather they want to enable screen clearing?
const SCREEN_CLEARING_CHOICES: &'static [&str;2] = &["Yes", "No"];

const MAIN_MENU_CHOICES: &'static [&str;5] = &[
    "Add Note",
    "Remove Note",
    "View Notes",
    "Generate Review",
    "Quit"
    ];

// Instantiated static during runtime
lazy_static! {
    // List of FG codes for colours
    // https://i.sstatic.net/9UVnC.png 
    static ref COLOURS: HashMap<&'static str, &'static str> = { 
        let mut map = HashMap::new();
        map.insert("RESET", "\x1B[0m");
        map.insert("RED", "\x1B[31m");
        map.insert("GREEN", "\x1B[32m");
        map
    };
}
static mut clear: bool = false;

#[derive(Error, Debug)]
enum main_error {
    #[error("{0}")]
    driver_error(String)
}
    
fn main() {
    // Enable screen clearing
    //let mut clear: bool = false;
    let clear_choice = Select::new()
        .with_prompt(format!("Enable screen clearning [\n({}Warning{} - Wipes current terminal",COLOURS["RED"],COLOURS["RESET"]))
        .items(SCREEN_CLEARING_CHOICES)
        .default(0)
        .interact()
        .unwrap();
    unsafe {
        match SCREEN_CLEARING_CHOICES[clear_choice] {
            "Yes" => clear = true,
            _ => clear = false,
    }
}
    // Load in notes
    println!("Loading...");
    let mut note_map: HashMap<String, Note> = HashMap::new();
    match load_map() {
        Ok(m) => note_map = m,
        Err(e) => panic!("Could not load note data, check config file and json file.\nError {e}"),
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
            "View Notes" => println!("woah"),
            "Generate Review" => println!("woah"),
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
        if clear {
            // Artifical wait so there is time to read the message before exit.
            thread::sleep(Duration::from_secs_f32(0.5));    
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
            println!("{}{}{}", COLOURS["GREEN"], message, COLOURS["RESET"]);
        },                    
        Err(e) => println!("{} {} {}",COLOURS["RED"], e, COLOURS["RESET"]),
    };
}


// Gets and adds a new Note to the note name
// Requests a name from the user, validates the name
// and creates as new Note, adding it to the map.
fn io_add_note(note_map: &mut HashMap<String, Note>) -> Result<String, main_error> {            
    let name = Input::new()
    .with_prompt("Enter the Notes Name")
    .validate_with(|input: &String| -> Result<(), &str> {
        match note_map.contains_key(&input.to_lowercase()) {
            false => Ok(()),
            true => Err("Note with same name already added\nTry Again"),
        }
    })
    .interact()
    .unwrap();

    let freq: u16 = 0;
    let last_accessed = "Today".to_string();

    note_map.insert(name.clone(), Note::new(name, freq, last_accessed));
    
    Ok("Success! Note Added".to_string())    
}

fn io_remove_note(note_map: &mut HashMap<String, Note>) -> Result<String, main_error> {
    Ok("Success! Note Removed".to_string())
}

