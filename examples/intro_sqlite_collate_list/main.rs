extern crate rusqlite;
use rusqlite::{Connection, Result};

struct Item {
    name: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let query = "SELECT name FROM pragma_collation_list() ORDER BY seq";

    let mut stmt = conn.prepare(&query)?;

    println!("\nSQLite collation names:");

    let rows = stmt.query_map([], |row| Ok(Item { name: row.get(0)? }))?;

    for item in rows {
        match item {
            Ok(i) => println!("    {}", i.name),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }

    Ok(())
}