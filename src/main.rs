extern crate rusqlite;

pub mod db;
use db::DB;

pub mod event;
use event::Event;

fn main() {

/*     let db = DB::open(String::from("./secretary.db")).unwrap();



    let tables: Result<Event, _> = db.query_row(
        "select * from mycal",
         &[],
        |row| -> Event {
            Event::new( 
                row.get(0),
                row.get(1),
                row.get(2)
            )

        }
    );
 */

    let db = DB::open("./secretary.db".to_owned()).unwrap();

    //create table mycal (name TEXT, desc TEXT, date DATE);
    db.db.execute_batch(
        "
        insert into mycal (name, desc, date) values ('Birthday', 'My birtday', '09/22/2018');
        insert into mycal (name, desc, date) values ('Vet', 'Vet appointment', '09/23/2018');
        "
    ).unwrap();

    let query = "select * from mycal".to_owned();

    println!("{:?}", db.get_tables());
}
