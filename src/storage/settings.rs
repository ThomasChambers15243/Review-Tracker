use std::fs;
use std::path::Path;

use crate::storage::StorageError;

pub fn get_json_path() -> Result<String, StorageError> {
    Ok("notes.json".to_string())
} 

pub fn valid_json_path() -> bool {
    Path::exists(Path::new("notes.json"))
}

pub fn create_json_file() -> Result<(), StorageError> {
    match fs::File::create_new("notes.json") {
        Ok(_) => Ok(()),
        Err(e) => Err(StorageError::File(format!("Could not create file. Error: {e}"))),
    }
}