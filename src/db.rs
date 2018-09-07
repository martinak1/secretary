// for sqlite interaction
use rusqlite::{Connection, Error, Statement};
use rusqlite::types::{ToSql};

// for path handeling
use std::path::PathBuf;

// for default path
use std::env;

// for events
//pub mod event;
use event::{self, Event};

//"insert into {} (name, date, desc) values ('{}' '{}' '{}')"

/// Struct that wraps around our DB
pub struct DB {
    pub path: String,
    pub db: Connection,
    pub cals: Vec<String>
}


impl DB {
    /// Opens the database at the given path, or creates it if it does not already exist
    // TODO add a check to see if a db exists -> if not create it and call add_cal
    pub fn open(path: String) -> Result<DB, Error> {
        let mut db = DB {
            path: path.clone(),
            db: Connection::open(path.as_str())?,
            cals: vec![]
        };

        // populate the cals vec
        db.set_cals()?;
        Ok(db)
    }
}

impl DB {
    /// Create a calendar table for a new database
    pub fn add_cal(&mut self, cal: String) -> Result<(), Error> {
/*         let sql = format!(

            );", cal);
 */

        //let mut stmnt = self.db.prepare(sql.as_str())?;
        let mut stmnt = self.db.prepare(
            "CREATE TABLE ?1 (
                id      INTEGER     PRIMARY KEY     AUTOINCREMENT   NOT NULL,
                cal     TEXT        NOT NULL, 
                name    TEXT        NOT NULL,
                desc    TEXT, 
                date    DATE        NOT NULL
        ")?;
        println!("Statement prepaired.");
        //println!("DEBUG: add_cal SQL:\n {}", sql);

        stmnt.execute(&[&cal.to_sql()?])?;
        self.cals.push(cal);

        Ok(())
    }

    /// Add events to the calendar
    pub fn add_events(&self, events: Vec<Event>) -> Result<(), Error> {

        for event in events.iter() {
/*             let stmnt = format!(
                "INSERT INTO secretary (cal, name, desc, date) VALUES ({}, {}, {}, {})",
                event.get_cal(),
                event.get_name(),
                event.get_desc(),
                event.get_date()
            );
            let mut stmnt: Statement = self.db.prepare(stmnt.as_str())?;
            stmnt.execute(&[])?;
*/

            let mut stmnt = self.db.prepare(
                "INSERT INTO secretary (cal, name, desc, date) VALUES (?1, ?2, ?3, ?4)"
            )?; 

            stmnt.insert(&[
                &event.get_cal().to_sql()?,
                &event.get_name().to_sql()?,
                &event.get_desc().to_sql()?,
                &event.get_date().to_sql()?
            ])?;

        }
        Ok(())
    }

    /// Deletes a calendar from the database
    pub fn drop_cal(&mut self, cal: String) -> Result<(), Error> {
        let mut stmnt = self.db.prepare(
            "DELETE FROM secretary WHERE cal = ?1"
        )?;

        let deleted = stmnt.execute(&[&cal.to_sql()?])?;

        println!("DEBUG: drop_cal removed {} entries.", deleted);

        Ok(())
    }

    /// Returns a Result holding a vector of events from the database
    // TODO Broken
    pub fn get_all_events(&self) -> Result<Vec<Event>, Error> {
        let mut stmnt = self.db.prepare("SELECT * FROM *")?;
        let mut rows = stmnt.query(&[])?;
        let mut events: Vec<Event> = vec![];

        while let Some(row) = rows.next() {
            let row = row?;
            events.push(Event::from_row(row)?);
        }

        Ok(events)
    }

    /// Returns a vector filled with table names
    pub fn get_cals(&self) -> &Vec<String> {
        &self.cals
    }


    /// Populates the 'cals' vector once the database connection has been established
    pub fn set_cals(&mut self) -> Result<(), Error> {
        let mut stmnt = self.db.prepare("select * from sqlite_master")?;
        let mut rows = stmnt.query(&[])?;

        while let Some(row) = rows.next() {
            let row = row?;
            self.cals.push(row.get(2));
        }

        println!("DEBUG: {:?}", self.cals);

        Ok(())

    }
}