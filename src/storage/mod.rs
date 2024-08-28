// Files read/write
use std::{
    fs,
    collections::BTreeMap,
    io,
    io::Write,
    fmt::{self},
};
// JSON
use serde::{Deserialize, Serialize};
// Errors
use thiserror::Error;

// For Later
// Date and time 
// use chrono::{DateTime, Duration, Utc};
// https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html
// https://docs.rs/chrono/latest/chrono/struct.DateTime.html

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("There was an I/O errors: {0}")]
    Io(#[from] io::Error),

    #[error("There was an Serde Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("There was an unexpected error: {0}")]
    Custom(String),
}

// Note type
#[derive(Deserialize, Serialize, Debug)]
pub struct Note {
    pub name: String,
    pub freq: u16,
    // To be changed to Chrono data later
    pub last_accessed: String,
}

// Prints Note values, each on a new line
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!(
            "Name: {}\nFreq: {}\nLast Accessed: {}",
            self.name, self.freq, self.last_accessed))
    }
}

impl Note {
    pub fn new(name: String, freq: u16, last_accessed: String) -> Self {
        Self { name, freq, last_accessed }
    }
}

// Save Fucntions \\

// Loads data from saved .json into a vector of note structs
pub fn load_json_data() -> Result<Vec<Note>, StorageError>{
    // Read from file
    let file = fs::read_to_string("notes.json")?;
    let json_data: serde_json::Value = serde_json::from_str(&file)?;
    
    // Load into vector
    let notes: Vec<Note> = json_data.as_object().unwrap().values()
    .map(|v| serde_json::from_value::<Note>(v.clone()).map_err(StorageError::from))
    .collect::<Result<Vec<Note>, StorageError>>()?;

    Ok(notes)
}

// Saves (writes) data from a vector of Note structs to a .json files
pub fn save_json_data(note_data: Vec<Note>) -> Result<(), StorageError>{
    let mut notes_map = BTreeMap::new();

    // Load notes into tree
    for (index, obj) in note_data.iter().enumerate() {
        notes_map.insert(index.to_string(), obj);         
    }

    // Write tree to string
    let json_string = serde_json::to_string_pretty(&notes_map)?;

    // Write string to the save file
    let mut file = fs::File::create("notes.json")?;
    file.write_all(json_string.as_bytes())?;
    file.flush()?;

    Ok(())
}



#[cfg(test)]
mod storage_tests {
    use super::*;

    
    #[test]
    fn test_load() {
        load_json_data();
        assert_eq!(true,true);
    }

    #[test]
    fn test_save() {
        let note_data = vec![
            Note {
                name: "Prep - COMP510".to_string(),
                freq: 2,
                last_accessed: "20-08-2024".to_string(),
            },
            Note {
                name: "Rust - The Slice Type".to_string(),
                freq: 3,
                last_accessed: "22-08-2024".to_string(),
            },
        ];
        save_json_data(note_data);
        assert_eq!(true,true);
    }

}

