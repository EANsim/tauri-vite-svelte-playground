use rusqlite::{params, Connection, Result};
use std::path::Path;
use log::{info, debug, error};

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

// Initialize the logger
fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS knop (
            id INTEGER PRIMARY KEY,
            block TEXT NOT NULL,
            types TEXT NOT NULL,
            name TEXT NOT NULL,
            state INTEGER NOT NULL,
            max_state INTEGER NOT NULL,
            comment TEXT NOT NULL
        )",
        [],
    ).map(|_| ()) // Map the usize result to ()
}

fn insert_knop(conn: &Connection, knop: &Knop) -> Result<(), rusqlite::Error> {
    info!("Inserting new knop: {:?}", knop);
    conn.execute(
        "INSERT INTO knop (block, types, name, state, max_state, comment) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![knop.block, knop.types, knop.name, knop.state, knop.max_state, knop.comment],
    ).map(|_| ()) // Map the usize result to ()
}

fn update_knop(conn: &Connection, knop: &Knop) -> Result<(), rusqlite::Error> {
    info!("Updating knop with id: {}", knop.id);
    conn.execute(
        "UPDATE knop SET block = ?1, types = ?2, name = ?3, state = ?4, max_state = ?5, comment = ?6 WHERE id = ?7",
        params![knop.block, knop.types, knop.name, knop.state, knop.max_state, knop.comment, knop.id],
    ).map(|_| ()) // Map the usize result to ()
}

fn delete_knop(conn: &Connection, id: i32) -> Result<(), rusqlite::Error> {
    info!("Deleting knop with id: {}", id);
    conn.execute("DELETE FROM knop WHERE id = ?1", params![id]).map(|_| ()) // Map the usize result to ()
}

fn select_all_knops(conn: &Connection) -> Result<Vec<Knop>> {
    let mut stmt = conn.prepare("SELECT id, block, types, name, state, max_state, comment FROM knop")?;
    let rows = stmt.query_map([], |row| {
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

    rows.collect()
}

fn main() -> Result<()> {
    init_logger(); // Initialize the logger

    let path = Path::new("knop.sqlite");
    let conn = Connection::open(path)?;

    create_table(&conn)?;

    // Insert a new Knop record
    let new_knop = Knop {
        id: 0, // SQLite will auto-increment this field
        block: "BlockA".into(),
        types: "Type1".into(),
        name: "Name1".into(),
        state: 1,
        max_state: 10,
        comment: "This is a new knop.".into(),
    };
    insert_knop(&conn, &new_knop)?;

    // Select all Knop records
    let all_knops = select_all_knops(&conn)?;
    debug!("All knops: {:?}", all_knops);

    // Update the first Knop record
    let updated_knop = Knop {
        id: 1, // Assuming the first inserted record has ID 1
        block: "UpdatedBlock".into(),
        ..new_knop // Keep other fields the same
    };
    update_knop(&conn, &updated_knop)?;

    // Delete the first Knop record
    delete_knop(&conn, 1)?;

    Ok(())
}
