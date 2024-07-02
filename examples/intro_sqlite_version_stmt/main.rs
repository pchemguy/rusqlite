extern crate rusqlite;
use rusqlite::{Connection, Result};

struct Item {
    version: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let mut stmt = conn.prepare("SELECT sqlite_version() AS version")?;
    let mut rows = stmt.query_map([], |row| {
        Ok(Item {
            version: row.get(0)?,
        })
    })?;

    println!("SQLite version:");

    for item in rows {
        match item {
            Ok(i) => println!("Version: {libversion}", libversion = i.version),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }


    stmt = conn.prepare("SELECT lower('щЩэЭюЮфФ') || upper('щЩэЭюЮфФ')")?;
    let mut rows = stmt.query_map([], |row| {
        Ok(Item {
            version: row.get(0)?,
        })
    })?;

    for item in rows {
        match item {
            Ok(i) => println!("CI Test: {text}", text = i.version),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }

    Ok(())
}
