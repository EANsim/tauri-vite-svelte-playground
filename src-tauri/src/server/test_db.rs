use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug)]
struct Knop {
    id: i32,
    block: String,
    types: String,
    name: String,
    state: i32,
    max_state: i32,
    comment: String,
}

fn main() -> Result<()> {
    let path = Path::new("states.sqlite");
    let conn = Connection::open(path)?;

/*
    conn.execute(
        "
CREATE TABLE person (
	id INTEGER PRIMARY KEY,
	block TEXT NOT NULL,
	types TEXT NOT NULL,
	name TEXT NOT NULL,
	state INTEGER,
	max_state INTEGER,
    comment TEXT NOT NULL
)
        ",
        (), // empty list of parameters.
    )?;
 */
    let me = Knop {
        id: 0,
        block: "test".to_string(),
        types: "лампа".to_string(),
        name: "q1".to_string(),
        state: 0,
        max_state: 2,
        comment: "test".to_string(),
    };
    /*
    conn.execute(
        "INSERT INTO person (block, types, name) VALUES (?1, ?2, ?3)",
        (&me.block, &me.types, &me.name),
    )?;*/

    let mut stmt = conn.prepare("SELECT id, block, types, name, state, max_state, comment FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Knop {
            id: row.get(0)?,
            block: row.get(1)?,
            types: row.get(2)?,
            name: row.get(3)?,
            state: row.get(4)?,
            max_state: row.get(5)?,
            comment: row.get(6)?,
        })
    })?;

    for person in person_iter {
        println!("Found {:?}", person.unwrap());
    }
    Ok(())
}
