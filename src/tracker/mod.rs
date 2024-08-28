use crate::storage::*;
use std::collections::HashMap;
use thiserror::Error;
use itertools::Itertools;

use lazy_static::lazy_static;
// Instantiated static during runtime
lazy_static! {
    // List of FG codes for ASCII
    // https://i.sstatic.net/9UVnC.png 
    // ASCII escape code
    // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    pub static ref ASCII: HashMap<&'static str, &'static str> = { 
        let mut map = HashMap::new();
        map.insert("RESET", "\x1B[0m");
        map.insert("BOLD", "\x1B[1m");
        map.insert("RED", "\x1B[31m");
        map.insert("GREEN", "\x1B[32m");
        map
    };
}

#[derive(Debug, Error)]
pub enum TrackerError{
    #[error("{0}")]
    StorageErr(#[from] StorageError),

    #[error("There was an unexpected error: {0}")]
    HashMap(String),

    #[error("There was an unexpected error: {0}")]
    Custom(String)
}

// Creates a hashmap of all notes
// from the saved json file
pub fn load_map() -> Result<HashMap<String, Note>, TrackerError>{
    // Loads data from storage
    let note_data: Vec<Note> = load_json_data()?;
    // Creates map with note names as keys
    let map: HashMap<String, Note> = note_data.into_iter()
    .map(|note| (note.name.clone(), note)).collect();
    Ok(map)
}

// Saves the current map
// to json file
pub fn save_map(map: HashMap<String, Note>) -> Result<(), TrackerError>{
    let note_data: Vec<Note> = map.into_iter().map(|note| note.1).collect();
    save_json_data(note_data)?;
    Ok(())
}

// Prints out the hash map of notes in an arbitary order
pub fn view_map(map: &HashMap<String, Note>) {
    for key in map.keys().sorted(){
        println!("{}{}{} has been reviewed {}{}{} times. Last reviewed, {}{}{}.",
                    ASCII["BOLD"], map[key].name, ASCII["RESET"],
                    ASCII["BOLD"], map[key].freq, ASCII["RESET"],
                    ASCII["BOLD"], map[key].last_accessed, ASCII["RESET"]
        );
    }
}


// Generates a vector of notes to review 
pub fn generate_review() {}

// Updates the note values map within map
pub fn update_reviewed_notes(notes: Vec<Note>) {}

pub fn manual_note_update(note: Note, freq: u16, last_accessed: String) { 
    std::unimplemented!("Waiting for date implementation")
}


// Review Calculations \\

fn find_least_common_notes(map: &HashMap<String, Note>) {}

fn find_oldest_notes(map: &HashMap<String, Note>) {}

fn calculate_time_difference(date_1: String, date_2: String) {
    std::unimplemented!("Waiting for date implementation")
}
