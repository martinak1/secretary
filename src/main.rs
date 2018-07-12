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

    fn create_table(cal: DB, month: String){
        cal.db.execute(format!(
            "CREATE TABLE {} (
            id      INTEGER PRIMARY KEY,
            date    INTEGER NOT NULL,
            name    TEXT NOT NULL,
            desc    TEXT    
            )", month),
            &[]
        ).unwrap();
    }

    // Insert values into July table
    // cal.db.execute(
    //     "INSERT INTO July (
    //         id, date, name, desc
    //      )",
    //      &[&0, &12, &"Work".to_string(), &"I do this five days a week".to_string()]
    // ).unwrap();

    fn insert_entry(cal: DB, month: String, entries: vec![]){
        cal.db.execute(format!(
            "INSERT INTO {} (
                id, date, name, desc
                )", month),
                &entries
        ).unwrap();
    }

    // Create the query
    let mut qresults = cal.db.prepare(
        "SELECT id, date, event, desc FROM July"
    ).unwrap();

    fn get_month(cal: DB, month: String) -> Result<Statement<'l>, Err>{
        let s = cal.db.prepare(format!("SELECT id, date, event, desc FROM {}", month))?;
        Ok(s)
    }

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
