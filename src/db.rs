extern crate rusqlite;
use rusqlite::*;

use std::path::PathBuf;

struct Event;

/// Struct that represents a SQLite database
struct DB {
    name: String,
    db:   Connection,
}

// functions
impl DB {
    /// Creates a new database
    fn new (path: PathBuf) -> Result<DB, Err> {

    }
    /// Opens and existing database 
    fn open (path: PathBuf) -> Result<DB, Err> {
        Ok( 
            DB {
                name: path.file_name(),
                db: Connection.open(path)?,
            }
        )
    }
}

// methods
impl DB {
    /// Save the database and closes it 
    fn close (&mut self) -> Result<(), Err> {
        self.db.commit()?;
        self.db.close()?;
        Ok(())
    }

    /// Save transactions to disk
    fn commit (&mut self) -> Result<(), Err> {
        self.db.commit()?;
    }

    /// Add and event to the calendar
    fn add_event (&mut self, event: Event) -> Result<(), Err> {

    }

    /// Drop an event from the calendar
    fn drop_event (&mut self, event: Event) -> Result<(), Err> {

    }

    /// Check for time confilts
    fn check_conflict(&mut self, event: Event) -> Option<Event> {

    }

    /// Create a view of a day
    fn view_day (&self, date: String) -> () {

    }

    /// Create a view of a month
    fn view_month (&self, date: String) -> () {

    }

    /// Create a view of a year
    fn view_year (&self, date: String) -> () {

    }
}
