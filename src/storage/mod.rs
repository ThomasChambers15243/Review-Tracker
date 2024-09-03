// Files read/write
use std::{
    collections::BTreeMap, 
    fmt::{self}, 
    fs,
    fs::File, 
    path::Path,
    io::{self, BufRead, Write},
};
// JSON
use serde::{Deserialize, Serialize};
// Errors
use thiserror::Error;


#[derive(Debug, Error)]
pub enum StorageError {
    #[error("There was an I/O errors: {0}")]
    Io(#[from] io::Error),

    #[error("There was an Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("There was  file error: {0}")]
    File(String),

    #[error("There was an unexpected error: {0}")]
    Custom(String),
}

// Note type
#[derive(Deserialize, Serialize, Debug, Clone)]
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

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
    // Check file path structure, if no file, create file
    if !valid_json_path() {
        create_json_file()?;        
    }
    
    let json_file_path = get_json_path()?;
    // Read from file
    let file = fs::read_to_string(json_file_path)?;

    // If file is empty
    if file.len() == 0 {
        Ok(vec![])
    } else {
        // Create json value
        let json_data: serde_json::Value = serde_json::from_str(&file)?;        
        // Load into vector
        let notes: Vec<Note> = json_data.as_object().unwrap().values()
        .map(|v| serde_json::from_value::<Note>(v.clone()).map_err(StorageError::from))
        .collect::<Result<Vec<Note>, StorageError>>()?;

        Ok(notes)
    }
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

pub fn get_note_names_from_file(path: &str) -> Result<Vec<String>, StorageError> {
    if !Path::exists(Path::new(path)) {
        return Err(StorageError::File("Could not find the file".to_string()));
    }
    let mut names: Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            names.push(line.trim().to_string());
        }
    };

    Ok(names)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
