use std::{collections::HashMap, io::{self, Write}, process};

use dialoguer::{theme::ColorfulTheme, Input, InputValidator, Select};
use storage::{load_json_data, Note};
use tracker::load_map;

pub mod storage;
pub mod tracker;

// User flag for weather they want to enable screen clearing?
const SCREEN_CLEARING_CHOICES: &'static [&str;2] = &["Yes", "No"];

const MAIN_MENU_CHOICES: &'static [&str;5] = &[
    "Add Note",
    "Remove Note",
    "View Notes",
    "Generate Review",
    "Quit"
    ];
    
    
fn main() {
    // Enable screen clearing
    let mut clear: bool = false;
    let clear_choice = Select::new()
        .with_prompt("Enable screen clearning [\n(\x1B[31mWarning\x1B[0m - Wipes current terminal")
        .items(SCREEN_CLEARING_CHOICES)
        .default(0)
        .interact()
        .unwrap();
    match SCREEN_CLEARING_CHOICES[clear_choice] {
        "Yes" => clear = true,
        _ => clear = false,
    }
    clear_screen(&clear);

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
        
        match MAIN_MENU_CHOICES[menu_choice] {
            "Add Note" => {println!("woah");clear_screen(&clear)},
            "Remove Note" => println!("woah"),
            "View Notes" => println!("woah"),
            "Generate Review" => println!("woah"),
            "Quit" => {            
                clear_screen(&clear); // Clear screen and reset cursor before exiting
                process::exit(0);
            },
            _ => {
                println!("Something went wrong, ending process");            
                clear_screen(&clear);
                process::exit(0);
            },
        };
    }
}



fn clear_screen(clear: &bool) {
    // Restore cursor to the saved position and clear everything below
    if *clear {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }
}

fn io_add_note() {

}

fn io_remove_note() {}

