// for sqlite interaction
use rusqlite::{self, Connection, Error, Statement};

// for path handeling
use std::path::PathBuf;

// for default path
use std::env;

// for events
//pub mod event;
use event::Event;


/// Struct that wraps around our DB
pub struct DB {
    path: String,
    db: Connection,
}


impl DB {
    /// Opens the database at the given path, or creates it if it does not already exist
    pub fn open(path: String) -> Result<Connection, Error> {
        Connection::open(path)
    }


}

impl DB {
    /// Returns a Result holding a vector of events from the database
    pub fn get_events(&self, sql: String) -> Result<Vec<Event>, Error> {
        let mut stmnt = self.db.prepare(sql.as_str())?;
        let mut rows = stmnt.query(&[])?;
        let mut events: Vec<Event> = vec![];

        while let Some(row) = rows.next() {
            let row = row.unwrap();
            events.push(
                Event::new(row.get(0), row.get(1), row.get(2))
            );
        }

        Ok(events)
    }
}