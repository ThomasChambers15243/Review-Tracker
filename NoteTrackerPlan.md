Note Tracker

Do

Tracks your notes and tells you when to review them, based upon their
frequency and time since last accessed.

Actions

- Can add notes
- Can remove notes
- Can view notes
- Can request generated review
- can manually review notes

How:

Collection of objects, stored permanently, with unique name-as-strings identifiers. Stored with JSON.

```json
	Name {
		name: String,
		freq: int,
		// Maybe Chrono data
		last accessed: date,
	}
```	

```Rust
Generate_Review(note_map: HasMap) {
		// Get 3 least common
		// Get 2 oldest
	
	}
```

Architecture
- Main - Driver code
- config - For details at a later point
- Mod
    - storage.rs
        - Object structs
        - JSON loading/writng
    - tracker.rs
        - Review generation
        - Review updatiung
        - reviews mangement
        

# ToDo
- Add config file path with json file
- Improve map view sort in tracker to sort by 
	1. alpha
	2. numerical
	3. ascii number
- Change note view to notes as a formated table	
# Bug List
- Loading Map
	- Errors when json is empty, causing cascading errors upwards.
    
