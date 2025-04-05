use rusqlite::{Connection, Result};
fn main() -> Result <()> {
    println!("Hello, world!");

    let conn = Connection::open("hotel_database.db")?;

    conn.execute (
        "CREATE TABLE IF NOT EXISTS hotel_chain (
            chain_name TEXT PRIMARY KEY,
            chain_addr TEXT NOT NULL,
            chain_email TEXT NOT NULL,
            chain_phone INT NOT NULL

        )",[],
    )?;

    Ok(())
}
