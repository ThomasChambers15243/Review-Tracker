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
---        
# Release Track
- [ ] Write read me
- [x] Can store notes, handle, view and review notes
- [x] Generate notes from .txt file of names per line
- [X] Get headers from dir containing .md files
- [X] Can mass delete from markdown and txt files
- [ ] Can edit note names
- [ ] Clean up code
- [ ] Include links
### Future track
- [ ] Save profiles for seperate note collections
- [ ] Markdown Note storage (obsidian wrapper?)
- [ ] OpenAI intergration to generate quetions based of notes
= [ ] SpellCheck - with https://norvig.com/spell-correct.html

# ToDo
- Can edit note names
- Reorganise code to clean up main
- Improve error system so everything isn't just MainError::DriverError(String)

- Improve map view sort in tracker to sort by 
	1. name
	2. Freq
	3. Date
- Documentation
- Critical testing
# Bug List
- Cannot exit note generator unless valid file is entered
