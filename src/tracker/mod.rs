use crate::storage::*;
use std::{clone, collections::HashMap};
use serde_json::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError{
    #[error("There was a storage error: {0}")]
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
    for (_, note) in map{
        println!("{} has been reviewed {} times. Last reviewed, {}.",
                    note.name, note.freq, note.last_accessed);
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
