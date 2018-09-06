// for sqlite interaction
use rusqlite::{self, Connection, Error, Statement, Rows};
use rusqlite::types::{ToSql, Type};

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
    pub tables: Vec<String>
}


impl DB {
    /// Opens the database at the given path, or creates it if it does not already exist
    pub fn open(path: String) -> Result<DB, Error> {
        let mut db = DB {
            path: path.clone(),
            db: Connection::open(path.as_str())?,
            tables: vec![]
        };

        // populate the tables vec
        db.set_tables()?;
        Ok(db)
    }


}

impl DB {

    /// Add events to the calendar
    pub fn add_events(&self, events: Vec<Event>) -> Result<(), Error> {

        for event in events.iter() {
            let mut stmnt: Statement = self.db.prepare(
                "insert into :table (name, desc, date) values (:name, :desc, :date)"
            )?;
            stmnt.execute_named(
                &[
                    (":table", &"mycal".to_owned()),
                    (":name", &event.get_name().to_sql()?),
                    (":desc", &event.get_desc().to_sql()?),
                    (":date", &event.get_date().to_sql()?)
                ]
            )?;

        }
        Ok(())

    }

    /// Returns a Result holding a vector of events from the database
    // TODO Broken
    pub fn get_events(&self, sql: String) -> Result<Vec<Event>, Error> {
        let mut stmnt = self.db.prepare(sql.as_str())?;
        let mut rows = stmnt.query(&[])?;
        let mut events: Vec<Event> = vec![];

        while let Some(row) = rows.next() {
            let row = row.unwrap();

            //println!("{:?} - {:?} - {:?}", row.get(0), row.get(1), row.get(2));
            events.push(
                //Event::new(row.get(0), row.get(1), row.get(2))
                Event::from_row(row)?
            );
        }

        Ok(events)
    }

    /// Returns a vector filled with table names
    pub fn get_tables(&self) -> &Vec<String> {
        &self.tables
    }

    /// Populates the 'tables' vector once the database connection has been established
    pub fn set_tables(&mut self) -> Result<(), Error> {
        let mut stmnt = self.db.prepare("select * from sqlite_master")?;
        let mut rows = stmnt.query(&[])?;

        while let Some(row) = rows.next() {
            let row = row?;
            self.tables.push(row.get(2));
        }

        println!("DEBUG: {:?}", self.tables);

        Ok(())

    }
}