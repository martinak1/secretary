// DB interactions
extern crate rusqlite;
use rusqlite::*;

struct DB {
    db: Connection,
    name: String,
}

#[derive(Debug)]
struct Event {
    id: i32,
    date: i32,
    name: String,
    desc: String,
}

fn main() {

    let cal = DB {
        db: Connection::open_in_memory().unwrap(),
        name: String::from("2018"),
    };

    // Create a table
    cal.db.execute(
        "CREATE TABLE July (
            id      INTEGER PRIMARY KEY,
            date    INTEGER NOT NULL,
            name    TEXT NOT NULL,
            desc    TEXT
         )",
         &[]
    ).unwrap();

    // Insert values into July table
    cal.db.execute(
        "INSERT INTO July (
            id, date, name, desc
         )",
         &[&0, &12, &"Work".to_string(), &"I do this five days a week".to_string()]
    ).unwrap();

    // Create the query
    let mut qresults = cal.db.prepare(
        "SELECT id, date, event, desc FROM July"
    ).unwrap();

    // Execute query and iter the results 
    let qriter = qresults.query_map(&[], |row| {
        Event {
            id:   row.get(0),
            date: row.get(1),
            name: row.get(2),
            desc: row.get(3),
        }
    }).unwrap();

    for event in qriter {
        println!(
            "{:?}", event.unwrap()
        );
    }

}
