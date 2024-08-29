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
        map.insert("BLUE", "\x1B[27m");
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
    println!("\n");
}


// Updates the note values map within map
pub fn update_reviewed_notes(note_map: &mut HashMap<String, Note>, reviewed: Vec<Note>) {
    for note in note_map.values_mut() {
        if reviewed.iter().any(|v| v == note) {
            note.freq += 1;
            note.last_accessed = "Today".to_string();
        }
    }
}

pub fn manual_note_update(note_map: Note, freq: u16, last_accessed: String) { 
    std::todo!("Write method")
}


// Review Calculations \\

pub fn find_least_common_notes(note_map: &HashMap<String, Note>, amount: usize) -> Vec<Note> {    
    let mut notes: Vec<&Note> = note_map.values().collect();

    notes.sort_by(|a, b| a.freq.cmp(&b.freq));

    notes.iter().take(amount).cloned().cloned().collect_vec()
}

fn find_oldest_notes(note_map: &HashMap<String, Note>) -> Vec<Note> {
    std::todo!("Write after date implementation")
}

fn calculate_time_difference(date_1: String, date_2: String) {
    std::todo!("Write after date implementation")
}
