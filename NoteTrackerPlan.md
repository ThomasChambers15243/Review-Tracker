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
		Frequency: int,
		last accessed: date,
	}
```	

```Rust
Generate_Review(num, default=5) {
		// num < =3
		// get least reviewed
		// else
		// get least reviewed
		// any gaps
		// get oldest reviwes
		// update records
	
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
- Improve map view sort in tracker to sort by 
	1. alpha
	2. numerical
	3. ascii number
# Bug List
- Loading Map
	- Errors when json is empty, causing cascading errors upwards.
    
