use crate::storage::*;
use std::collections::HashMap;
use chrono::TimeDelta;
use thiserror::Error;
use itertools::Itertools;
use chrono::prelude::*;

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

    #[error("There was an DateTime error: {0}")]
    DateTime(String),

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
            note.last_accessed = Local::now().to_string();
        }
    }
}

#[allow(unused)]
pub fn manual_note_update(note_map: HashMap<String, Note>, freq: u16, last_accessed: String) { 
    std::todo!("Write method")
}


// Review Calculations \\

pub fn find_least_common_notes(note_map: &HashMap<String, Note>, amount: usize) -> Vec<Note> {    
    let mut notes: Vec<&Note> = note_map.values().collect();

    notes.sort_by(|a, b| a.freq.cmp(&b.freq));

    notes.iter().take(amount).cloned().cloned().collect_vec()
}
#[allow(unused)]
fn find_oldest_notes(note_map: &HashMap<String, Note>) -> Vec<Note> {
    let mut oldest: Vec<&Note> = note_map.values().collect();

    std::todo!();
}

#[allow(unused)]
fn calculate_time_difference(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> Result<HashMap<String, i64>, TrackerError>{
    if dt1 < dt2 {
        return Err(TrackerError::DateTime("Date 1 was smaller than data 2, it should be larger".to_string()));
    }
    // Holds years, months, days, hours, minutes, seconds
    let mut duration_since: HashMap<String, i64> = HashMap::new();
    let difference: TimeDelta = dt1 - dt2;
    
    duration_since.insert("days".to_string(), difference.num_days());
    duration_since.insert("hours".to_string(), difference.num_hours());
    duration_since.insert("minutes".to_string(), difference.num_minutes());
    duration_since.insert("seconds".to_string(), difference.num_seconds());

    Ok(duration_since)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_diff() {
        let dt1: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        let dt2: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 6, 25, 15, 30, 45).unwrap();

        let diff = calculate_time_difference(dt1, dt2).unwrap();
        println!("Days: {}",diff["days"]);
        println!("Hours: {}",diff["hours"]);
        println!("Minutes: {}", diff["minutes"]);
        println!("Seconds: {}", diff["seconds"]);
    }
}
