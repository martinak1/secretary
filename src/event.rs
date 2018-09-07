use rusqlite::{Error, Row};

/// A representation of an event 
#[derive(Debug)]
pub struct Event {
    id:   Option<i64>,
    cal: String,
    name: String,
    desc: String,
    date: String,
}


// functions
impl Event {
    
    /// Create a new event
    pub fn new(cal: String, name: String, desc: String, date: String) -> Event {
        Event {
            id: None,
            cal,
            name,
            desc,
            date
        }
    }

    pub fn from_row(row: Row) -> Result<Event, Error> {
        Ok (
            Event {
                id: Some(row.get(0)),
                cal: "mycal".to_owned(),
                name: row.get(1),
                desc: row.get(2),
                date: row.get(3)
            }
        )
    }



}


// methods
impl Event {

    /// Return the event ID
    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    /// Return the event name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Return the event description
    pub fn get_desc(&self) -> &String {
        &self.desc
    }

    /// Return the date of the event
    pub fn get_date(&self) -> &String {
        &self.date
    }

    /// Return the calander the event is held in
    pub fn get_cal(&self) -> &String {
        &self.cal
    }
}