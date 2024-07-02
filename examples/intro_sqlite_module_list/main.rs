extern crate rusqlite;
use rusqlite::{Connection, Result};

struct Item {
    name: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let query = "SELECT name FROM pragma_module_list() ORDER BY name";

    let mut stmt = conn.prepare(&query)?;

    println!("\nSQLite module names:");

    let rows = stmt.query_map([], |row| Ok(Item { name: row.get(0)? }))?;

    for item in rows {
        match item {
            Ok(i) => println!("    {}", i.name),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }

    stmt = conn.prepare("SELECT lower('щЩэЭюЮфФ') || upper('щЩэЭюЮфФ')")?;
    let rows = stmt.query_map([], |row| {
        Ok(Item {
            name: row.get(0)?,
        })
    })?;

    for item in rows {
        match item {
            Ok(i) => println!("CI Test: {text}", text = i.name),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }

    Ok(())
}
