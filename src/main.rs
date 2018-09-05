extern crate rusqlite;

pub mod db;
use db::DB;

pub mod event;
use event::Event;

fn main() {

/*     let db = DB::open(String::from("./secretary.db")).unwrap();

     db.execute(
        "
        create table mycal (name TEXT, desc TEXT, date DATE);
        insert into mycal (name, date, desc) values ('My Birthday', '09/22/2018', 'My Birthday');
        insert into mycal (name, date, desc) values ('Jaleesa's Birthday', '10/22/2018', 'Jaleesa's Birthday');
        ",
        &[]
    ).unwrap();

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
}
