use std::{collections::HashMap, ffi::OsStr, io::{self, Write}, process};

use chrono::{Local, Utc};
// Crates
use dialoguer::{theme::ColorfulTheme, Input, Select};

// Mods
pub mod storage;
pub mod tracker;
use itertools::Itertools;
use storage::*;
use thiserror::Error;
use tracker::*;
use walkdir::WalkDir;


// Choice menus
const YES_NO_CHOICES: &'static [&str;2] = &["YES", "NO"];

const MAIN_MENU_CHOICES: &'static [&str;6] = &[
    "Add Note",
    "Remove Note",
    "View Notes",
    "Generate Review",
    "Generate Notes",
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
                    Ok(_) => io_view_map(&note_map),
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
            "Generate Notes" => {
                handle_map_operation(&mut note_map, |m| io_generate_notes(m));
            }
            _ => {
                println!("Something went wrong, ending process");            
                clear_screen();
                process::exit(0);
            },
        };
    }
}

fn sync_map(note_map: HashMap<String, Note>) -> Result<HashMap<String, Note>, MainError>{
    match save_map(note_map) {
        Err(e) => Err(MainError::DriverError(e.to_string())),
        _ => match load_map() {
                Ok(m) => Ok(m),
                Err(e) => Err(MainError::DriverError(
                format!("Error: Could not load note data, check config file and json file. \nError {e}"
                ))),
        }
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

fn io_view_map(note_map: &HashMap<String, Note>) {
    println!("{}",bold_wrap!("...Notes..."));
    for key in note_map.keys().sorted(){
        println!("Note {} 
\tReviewed: {} times.
\tLast reviewed: {},
\tTime Since: {}",
bold_wrap!(note_map[key].name),
bold_wrap!(note_map[key].freq),
bold_wrap!(format_time_for_output(&note_map[key].last_accessed)),
bold_wrap!(format_time_since(&note_map[key].last_accessed).unwrap())
        );
    }
    //println!("\n");
}

fn io_generate_notes(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {

    let choices = ["Markdown directory", "Markdown file (.md)", "Text file (.txt)"];    

    let choice = Select::new()
        .with_prompt("Select where you would like to generate new notes from")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    match choices[choice] {
        "Markdown directory" => { 
            // List of all markdown files found, if emtpy, none found. Maybe invalid root name.
            let mut files_found: Vec<String> = Vec::new();
            let root = io_get_file_path("");            
            for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {                    
                    match path.extension().and_then(OsStr::to_str) {
                        Some("md") => {
                            if let Ok(_) = io_get_notes_from_markdown(path.to_str().unwrap().to_string(), note_map) {
                                files_found.push(path.file_name().unwrap().to_str().unwrap().to_string());
                            }
                        },
                        _ => (),
                    };
                }
            }
            // If not dir is found with given file path
            if !files_found.is_empty() {
                Ok(format!("Notes added from files:\n{}",files_found.join("\n")))
            } else {
                Err(MainError::DriverError("Could not Find directory".to_string()))
            }
        },
        // Make down and text use the same code but with
        "Markdown file (.md)" => {
            // Gets file path
            let file_path = io_get_file_path("");
            match io_get_notes_from_markdown(file_path, note_map) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(e),
            }
        },
        "Text file (.txt)" => {
            // Gets file path
            let file_path = io_get_file_path(".txt");
            // Gets note names
            match get_note_names_from_file(file_path.as_str()) {
                Ok(note_names) => {
                    io_create_new_notes_from_vec(note_names, note_map);      
                    Ok(format!("New Notes successfully added from file {}", bold_wrap!(file_path)))
                },
                Err(e) => Err(MainError::DriverError(format!(
                    "Could not get names, due to error: {e}"))),
            }
        },
        _ => Ok("What option was this???".to_string()),
    }


}


// Gets and adds a new Note to the note name
// Requests a name from the user, validates the name
// and creates as new Note, adding it to the map.
fn io_add_note(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {            
    // Notes values
    let freq: u16 = 0;
    let last_accessed = Local::now().to_string();    
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

    let (mut uncommon, mut oldest) = get_notes_to_review(note_map);

    // Formats and prints Notes to Review \\ 

    format_review(&uncommon, &oldest);

    
    // Save Review
    let save = Select::new()
    .with_prompt("Save Review?")
    .items(YES_NO_CHOICES)
    .default(0)
    .interact()
    .unwrap();

match YES_NO_CHOICES[save] {
    "YES" => {
            // Join together two sets of notes and update them in the json file
            uncommon.append(&mut oldest);
            update_reviewed_notes(note_map, uncommon);
            Ok("Notes Saved".to_string())
        },
        _ => Err(MainError::DriverError("Notes were not saved".to_string()))
    }
}


fn io_handle_empty_map(note_map: &HashMap<String, Note>) -> Result<String, MainError> {
    // Check map is empty
    match note_map.is_empty() {
        true => Err(MainError::DriverError(format!(
                "{}No notes to review\nTry adding some notes with {}",
                ASCII["GREEN"], bold_wrap!("Add Note")).to_string())),
        false => Ok("".to_string()),
    }
}


fn io_get_file_path(file_type: &str) -> String{
    // Gets file path
    Input::new()
    .with_prompt("Enter the file path")
    .validate_with(|input: &String| -> Result<(), &str> {
        if input.chars().count() < file_type.len() {
            println!("File Path must end with {}", file_type);
            return Err("Try again");
        }    
        // Reverses string a chars, takes the first 4, then reverses it again    
        let last_four: String = input.chars().rev().take(file_type.len()).collect::<Vec<_>>().into_iter().rev().collect();            
        if last_four != file_type {
            println!("File Path must end with {}", file_type);
            Err("")
        } else {
            Ok(())
        }
    })
    .interact()
    .unwrap()
}

fn io_create_new_notes_from_vec(note_names: Vec<String>, note_map: &mut HashMap<String, Note>) {    
    for name in note_names {
        note_map.insert(name.clone(), Note::new(name, 0, Local::now().to_string()));
    }            
}

fn io_get_notes_from_markdown(file_path: String, note_map: &mut HashMap<String, Note>) -> Result<String, MainError>{
    let markdown_choices = ["H1 (#)", "H2 (##)", "H3 (###)", "H4 (####)", "H5 (#####)", "H6 (######)"];
    // Gets the min_hashes for markdown parsing
    let header_length = Select::new()
    .with_prompt(format!("Whats the smallest header type you would like to include for file:{}", file_path))
    .items(&markdown_choices)
    .default(0)
    .interact()
    .unwrap();
    // Gets header names
    match get_note_names_from_markdown(file_path.as_str(), header_length+1) {
        Ok(note_names) => {
            io_create_new_notes_from_vec(note_names, note_map);      
            Ok(format!("New Notes successfully added from file {}", bold_wrap!(file_path)))
        },
        Err(e) => Err(MainError::DriverError(format!(
            "Could not get names, due to error: {e}"))),
    }
}