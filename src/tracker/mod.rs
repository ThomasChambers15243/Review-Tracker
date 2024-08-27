use crate::storage::*;
use std::collections::{hash_map, HashMap};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError{
    #[error("There was an unexpected error: {0}")]
    HashMap(String)
}

// Creates a hashmap of all notes
// from the saved json file
pub fn load_map() {}

// Saves the current map
// to json file
pub fn save_map() {}

// Prints out the hash map
pub fn view_map() {}

// Searches the map for a given note
pub fn search_map(map: &HashMap<String, Note>, target: &str) {}

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
