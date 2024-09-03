use crate::storage::*;
use std::collections::HashMap;
use chrono::{ParseError, TimeDelta};
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


#[macro_export]
macro_rules! bold_wrap {
    ($single:expr) => {
        format!("\x1B[1m{}\x1B[0m", $single)
    };
    ($first:expr, $($rest:expr),+) => {
        {
            let mut string = String::from($first);
            $(
                string.push_str(", ");
                string.push_str($rest.to_string().as_str());
            )+
            format!("\x1B[1m{}\x1B[0m", string)
        }
    };
}



#[derive(Debug, Error)]
pub enum TrackerError{
    #[error("{0}")]
    StorageErr(#[from] StorageError),

    #[error("There was an unexpected error: {0}")]
    HashMap(String),

    #[error("There was an DateTime formatting error: {0}")]
    DateTimeFormatting(String),

    #[error("{0}")]
    DateTime(#[from] ParseError),

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
        println!("{}{}{} has been reviewed {}{}{} times. Last reviewed, {}{}{},\n{} ago.",
                    ASCII["BOLD"], map[key].name, ASCII["RESET"],
                    ASCII["BOLD"], map[key].freq, ASCII["RESET"],
                    ASCII["BOLD"], format_time_for_output(&map[key].last_accessed), ASCII["RESET"],
                    format_time_since(&map[key].last_accessed).unwrap()
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

pub fn get_notes_to_review(note_map: &HashMap<String, Note>) -> (Vec<Note>, Vec<Note>) {
    let mut notes: Vec<&Note> = note_map.values().collect();

    // Sort by freq
    notes.sort_by(|a, b| a.freq.cmp(&b.freq));
    // Gets 3 most uncommon 
    let uncommon = notes.iter().take(3).cloned().cloned().collect_vec();    
    
    // Sort by date
    notes.sort_by(|a, b| {
        let a_time = DateTime::parse_from_str(
            &a.last_accessed,
            "%Y-%m-%d %H:%M:%S%.9f %z").unwrap();
        let b_time = DateTime::parse_from_str(
            &b.last_accessed,
            "%Y-%m-%d %H:%M:%S%.9f %z").unwrap();
        a_time.cmp(&b_time)
    });
    // Gets 2 oldest that arn't already in the most uncommon vec
    let olderst = notes.iter()
        .filter(|n| !uncommon.contains(n))
        .take(2).cloned().cloned().collect_vec();

    (uncommon, olderst)
}


pub fn format_review(uncommon: &Vec<Note>, oldest: &Vec<Note>) {
    // Title
    println!("{}\n",bold_wrap!("...Notes to Review..."));
    
    // Least common    
    println!("{}", bold_wrap!("Least Reviewed:"));
    for note in uncommon {     
        println!("Note: {} - Reviewed {} times, last at {}",
            bold_wrap!(note.name),
            bold_wrap!(note.freq),
            bold_wrap!(format_time_for_output(&note.last_accessed))
        );
    }

    // Oldest
    println!("\n{}",bold_wrap!("Oldest Since Last Review:"));

    for note in oldest {        
        println!("Note: {} - Last reviewed at: {} times, Last at {}\n\tTime since review: {}",
            bold_wrap!(note.name),
            bold_wrap!(note.freq),
            bold_wrap!(format_time_for_output(&note.last_accessed)),
            bold_wrap!(format_time_since(&note.last_accessed).unwrap())
        );
    }

    // Create gap between next select
    println!("\n");
}


// Calculate the time differenc between two DateTime<Utc> dates, dt1 must be larger than dt2 else error
fn calculate_time_difference(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> Result<HashMap<String, i64>, TrackerError>{
    if dt1 <= dt2 {
        return Err(TrackerError::DateTimeFormatting("Date 1 was smaller than data 2, it should be larger".to_string()));
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



pub fn format_time_for_output(time: &str) -> String {    
    let utc_time = DateTime::parse_from_str(
        time,
        "%Y-%m-%d %H:%M:%S%.9f %z").unwrap();
    
    format!("{}", utc_time.format("%Y-%m-%d %H:%M:%S"))
}

pub fn format_time_since(time: &str) -> Result<String, TrackerError> {
    
    let date: DateTime<Utc> = DateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S%.9f %z")?.into();
    
    match calculate_time_difference(Utc::now(), date) {
        Ok(mut map) => {
            // Filter out times with no values
            // and sort order from days to seconds
            let mut diff: Vec<(&String, &mut i64)> = map.iter_mut()
                .filter(
                    |(_,&mut v)| 
                    v != 0
                )
                .collect_vec()
                .into_iter().sorted_by(
                    |a, b| 
                    Ord::cmp(a.1,b.1)
                )
                .collect_vec();

            // Reduce added time so its an true representation of how long
            let mut seen_time = 0;
            for (date, dur) in  diff.iter_mut() {
                match date.as_ref() {
                    "days" => seen_time = **dur * 24,
                    "hours" => {
                        **dur -= seen_time;
                        seen_time *= 60;
                        seen_time += **dur * 60;
                    },
                    "minutes" => {
                        **dur -= seen_time;
                        seen_time *= 60;
                        seen_time += **dur * 60;
                    }
                    "seconds" => {
                        **dur -= seen_time;
                    }
                    _ => return Err(TrackerError::DateTimeFormatting("Unknown Time name in list".to_string())),
                }
            }
            // Convert vector to single string
            let mut time_string = String::new();
            for (date, dur) in diff.iter() {
                time_string.push_str(format!("{date}: {dur}, ").as_str());
            }
            Ok(time_string)
        },
        Err(e) => Err(TrackerError::DateTimeFormatting(format!("{e}"))),
    }
}
