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
