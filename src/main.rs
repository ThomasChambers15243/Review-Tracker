use std::{collections::HashMap, ffi::OsStr, io::{self, Write}, process};

// Crates
use dialoguer::{theme::ColorfulTheme, Input, Select};
use itertools::Itertools;
use chrono::Local;
use thiserror::Error;
use walkdir::WalkDir;

// Mods
pub mod storage;
pub mod tracker;
use storage::*;
use tracker::*;


// Choice menus
const YES_NO_CHOICES: &'static [&str;2] = &["YES", "NO"];

const MAIN_MENU_CHOICES: &'static [&str;8] = &[
    "Add Note",
    "View Notes",
    "Edit Note",
    "Remove Note",
    "Generate Review",
    "Generate Notes",
    "Remove Notes Using File",
    "Quit"
    ];

// Boolean flag to indicate whether the user want to clear the screen after inputs or not
static mut CLEAR: bool = false;

#[derive(Error, Debug)]
enum MainError {
    #[error("{0}")]
    DriverError(String),

    #[error("{0}")]
    TrackerError(#[from] TrackerError),

    #[error("{0}")]
    StorageError(#[from] StorageError)
}
    
fn main() {
    // Loads in notes
    println!("Loading...");

    // Main map of notes used throughout.
    // If it cannot be loaded, the process is ended :(
    let mut note_map: HashMap<String, Note>;
    match load_map() {
        Ok(map) => note_map = map,
        Err(e) => {
            println!("Could not load map due to error {}\nEnding Process...",e);
            process::exit(1);
        },
    }

    // Enable screen clearing
    let clear_choice = select_wrapper(
        format!("Enable screen clearning\n{} - Wipes current terminal", red_wrap!("Warning")).as_str(),
        YES_NO_CHOICES);
    // Unsafe as due to modification of mutable static
    // This is the only time theres any change so its chill
    unsafe {
        match YES_NO_CHOICES[clear_choice] {
            "YES" => CLEAR = true,
            _ => CLEAR = false,
        }
    }

    // Main Loop
    clear_screen();
    loop {
        // Saves map at the start of every loop, this might be changed later
        // depending on performance costs.
        // Ends process if it cannot sync (it always should if it can at the start)
        match sync_map(note_map) {
            Ok(map) => note_map = map,
            Err(e) => {
                println!("Could not save and sync the map due to error{}\nEnding Process...",e);
                process::exit(1);
            },
        }

        // Gets main menu choice from user
        let menu_choice = select_wrapper(
            "Main Menu",
            MAIN_MENU_CHOICES
        );
    
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
                handle_map_operation(&mut note_map, |m| io_view_map(&m));
            },
            "Edit Note" => {
                handle_map_operation(&mut note_map, |m| io_edit_note_map(m));
            },
            "Generate Review" => {
                handle_map_operation(&mut note_map, |m| io_generate_review(m));
            },
            "Generate Notes" => {
                handle_map_operation(&mut note_map, |m| io_generate_notes(m));
            },
            "Remove Notes Using File" => {
                handle_map_operation(&mut note_map, |m| io_remove_notes_wth_file(m));
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

// Restore cursor to the saved position and clear everything below
fn clear_screen() {
    // Unsafe as CLEAr is a mutabable static, however, its only ever changed once 
    // at the start of the main loop. It should never change again, this should
    // always work
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

// Prints out each note in map, formatted along with note details
fn io_view_map(note_map: &HashMap<String, Note>) -> Result<String, MainError>{
    io_handle_empty_map(note_map)?;
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
    Ok("".to_string())
}

// Input/Output options and handling for generating notes from markdown directorys,
// markdown files or .txt files
fn io_generate_notes(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    let choices = ["Markdown directory", "Markdown file (.md)", "Text file (.txt)"];    
    let choice = select_wrapper("Select where you would like to generate new notes from", &choices);    

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
                    let prefix = io_get_prefix();
                    io_create_new_notes_from_vec(prefix, note_names, note_map);      
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
    let sure = select_wrapper("Add Note?", YES_NO_CHOICES);

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
    let sure = select_wrapper("Remove Note?", YES_NO_CHOICES);

    // Gives user an out incase they're filled with a deep regret over
    // their note choice
    match YES_NO_CHOICES[sure] {
        "YES" => {
            io_del_note(name, note_map)
        },
        _ => Err(MainError::DriverError("No Note was removed".to_string())),
    }
    
}

// Given a .txt or .md files, removes matching names
fn io_remove_notes_wth_file(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    let file_types = ["Markdown (.md)", "Text (.txt)"];
    let choice = select_wrapper("Select file type", &file_types);

    match file_types[choice] {
        "Markdown (.md)" => {
            let file_path = io_get_file_path(".md");
            // Gets the min_hashes for markdown parsing
            let markdown_choices = ["H1 (#)", "H2 (##)", "H3 (###)", "H4 (####)", "H5 (#####)", "H6 (######)"];
            let header_length = select_wrapper(
                format!("Whats the smallest header type you would like to include for file:{}", file_path).as_str(),
                        &markdown_choices
            );            
            let names = get_note_names_from_markdown(file_path.as_str(), header_length)?;
            let prefix = io_get_prefix();
            
            for name in names {
                let mut note_name = prefix.clone();
                note_name.push_str(name.as_str());
                match io_del_note(note_name, note_map) {
                    Ok(msg) => println!("{}", green_wrap!(msg)),
                    Err(msg) => println!("{}", red_wrap!(msg)),
                }
            }
            Ok("Any notes found were removed...".to_string())
        },
        "Text (.txt)" => {
            let file_path = io_get_file_path(".txt");

            match get_note_names_from_file(file_path.as_str()) {
                Ok(names) => {
                    let prefix = io_get_prefix();
                    for name in names {
                        let mut note_name = prefix.clone();
                        note_name.push_str(name.as_str());
                        match io_del_note(note_name, note_map) {
                            Ok(msg) => println!("{}", green_wrap!(msg)),
                            Err(msg) => println!("{}", red_wrap!(msg)),
                        }
                    }
                    Ok("Any notes found were removed...".to_string())
                },
                Err(e) => Err(e.into()),
            }
        },
        _ => Ok("This errr...his wasn't an option? How did you...oh..OH MY GOD NO PUT IT DOWN!! SOMEONE HELP, WHY ME NO PLZ PFHDSUDIK...".to_string())
    }
}

// Handles the review, getting the notes to review, fomratting their display and upadting 
// the notes's values.
fn io_generate_review(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    // Handle case where map is empty
    io_handle_empty_map(note_map)?;

    let (mut uncommon, mut oldest) = get_notes_to_review(note_map);

    // Formats and prints Notes to Review \\ 

    format_review(&uncommon, &oldest);

    // Save Review
    let save = select_wrapper("Save Review?", YES_NO_CHOICES);

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


// Handle error handling wheen map is empty
fn io_handle_empty_map(note_map: &HashMap<String, Note>) -> Result<String, MainError> {
    if !note_map.is_empty() {
        Ok("".to_string())
    } else {
        Err(MainError::DriverError(format!("{}{}",
        green_wrap!("No notes to review\nTry adding some notes with "),
        bold_wrap!("Add Note")).to_string()))
    }
}

// Gets and validates files path accoring to given types
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

// Given a vector of strings and a prefix, inserts the prefix to each name and creates a new note with
// said name into the map
fn io_create_new_notes_from_vec(prefix: String, note_names: Vec<String>, note_map: &mut HashMap<String, Note>) {
    for name in note_names {
        let mut note_name: String = prefix.clone();
        note_name.push_str(name.as_str());
        note_map.insert(note_name.clone(), Note::new(note_name, 0, Local::now().to_string()));
    }            
}

// Get notes from header names in a markdown file, according to the given header level, and adds it to the map
fn io_get_notes_from_markdown(file_path: String, note_map: &mut HashMap<String, Note>) -> Result<String, MainError>{
    let markdown_choices = ["H1 (#)", "H2 (##)", "H3 (###)", "H4 (####)", "H5 (#####)", "H6 (######)"];
    // Gets the min_hashes for markdown parsing
    let header_length = select_wrapper(
        format!("Whats the smallest header type you would like to include for file:{}", file_path).as_str(),
    &markdown_choices
    );
    // Gets header names
    match get_note_names_from_markdown(file_path.as_str(), header_length+1) {
        Ok(note_names) => {
            let prefix = io_get_prefix();
            io_create_new_notes_from_vec(prefix, note_names, note_map);      
            Ok(format!("New Notes successfully added from file {}", bold_wrap!(file_path)))
        },
        Err(e) => Err(MainError::DriverError(format!(
            "Could not get names, due to error: {e}"))),
    }
}

// User options to get a prefix string
fn io_get_prefix() -> String {
    let is_prefix = select_wrapper(
        "Would you like to add a prefix to the names?\n([prefix][NoteName])", 
        YES_NO_CHOICES);
    match YES_NO_CHOICES[is_prefix] {
        "YES" => {
            Input::new()
            .with_prompt(format!("Enter prefix\n{}", 
                bold_wrap!("Enter a seperator if desired, otherwise none are added")))
            .interact()
            .unwrap()
        },
        _ => "".to_string()
    }
}

// Wrapper around dialoger's Select struct
fn select_wrapper<T: ToString>(prompt: &str, items: &[T]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .items(items)
    .default(0)
    .interact()
    .unwrap()
}

// Wrapper around dialoger's Input struct
fn input_wrapper(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact()
                .unwrap()
}

// Delete note with given name from map
fn io_del_note(name: String, note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    if let Some(note_name) = note_map.keys()
    .find(|key| key.to_lowercase() == name.to_lowercase()).cloned()
    {
        if let Some(note) = note_map.remove(&note_name) {
            return Ok(format!(
                "{} was removed with values:\nFreq: {}\nLast Accessed: {}",
                note.name, note.freq, note.last_accessed
            ));
        } else {
            return Err(MainError::DriverError(format!("Could not find note to remove of name {}", bold_wrap!(name))));
        }
    }
    Err(MainError::DriverError(format!("Could not find note to remove of name {}", bold_wrap!(name))))
}

// Edit note with given name from map
fn io_edit_note_map(note_map: &mut HashMap<String, Note>) -> Result<String, MainError> {
    io_handle_empty_map(note_map)?;
    let search_option = ["Search", "Selection"];
    let choice = select_wrapper("Search by name or selection", &search_option);
    match search_option[choice] {
        "Search" => {
            let name: String = input_wrapper("Enter Note Name");
            // If note exists, display attributes and give user options
            // for editing notes name and freq
            if let Some(note_name) = find_note_name(&name, note_map) {
                let note: &mut Note = note_map.get_mut(&note_name).unwrap();
                println!("Name: {}\nFreq: {}\nLast Accessed: {}",
                    bold_wrap!(note.name),
                    bold_wrap!(note.freq),                    
                    bold_wrap!(format_time_for_output(&note.last_accessed))
                );
                io_edit_note(note);            
                Ok("Note was updated".to_string())
            } else {
                Err(MainError::DriverError("Couldn't find note".to_string()))
            }

        },
        "Selection" | _ =>  {
            io_select_all_note(note_map);
            Ok("".to_string())
        }
    }

}


// Opens editing an idividual note for the user
fn io_edit_note(note: &mut Note) {
    let attr = ["Name", "Freq", "Save"];
    loop {        
        // Edit Note
        match attr[select_wrapper("What would you like to edit?", &attr)] {
            "Name" => {
                note.name = input_wrapper("Enter new name");
                println!("{}", green_wrap!(format!("Name set to {}", bold_wrap!(note.name))));
            },
            "Freq" => {
                // Unwrap will always pass due to validator
                note.freq = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the freq")
                        .validate_with(|input: &String| -> Result<(),&str> {
                            match input.parse::<u16>() {
                                Ok(_) => Ok(()),
                                Err(_) => Err("Must enter a positive number"),
                            }
                        })
                        .interact()
                        .unwrap().parse::<u16>().unwrap();
                println!("{}", green_wrap!(format!("Note Freq set to {}", bold_wrap!(note.freq))));
            },
            "Save" | _ => {
                return
            }
       }
    }
}

// Input for browsing through all notes
fn io_select_all_note(note_map: &mut HashMap<String, Note>) {
    let mut all_notes: Vec<&mut Note> = note_map.values_mut().collect();    
    loop {
        let choice = select_wrapper("prompt", &all_notes);
        io_edit_note(all_notes[choice]);
        match YES_NO_CHOICES[select_wrapper("Edit Another?", YES_NO_CHOICES)] {
            "YES" => (),
            _ => return,
        }
    }
    
}

// Finds the note in the map if it exists
fn find_note_name(name: &str, note_map: &mut HashMap<String, Note>) -> Option<String> {
    if let Some(note_name) = note_map.keys()
        .find(|key| key.to_lowercase() == name.to_lowercase()).cloned()
    {
        Some(note_name)
    } else {
        None
    }
}