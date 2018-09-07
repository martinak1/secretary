extern crate rusqlite;

pub mod db;
use db::DB;

pub mod event;
use event::Event;

fn main() {
    test();
}

fn test() {
    let mut db = DB::open("./secretary.db".to_owned()).unwrap();

    db.add_cal("testdb".to_owned()).unwrap();

    println!("{:?}", db.get_cals());

    let events = vec![
        Event::new(
           "functiontest".to_owned(),
           "Birthday".to_owned(),
           "Its my birthday".to_owned(),
           "09/22/2018".to_owned(),
        ),
        Event::new(
            "functiontest".to_owned(),
            "Work".to_owned(),
            "I do this a lot".to_owned(),
            "09/20/2018".to_owned(),
        )
    ];

    // TODO broken
    db.add_events(events).unwrap();

    let events = db.get_all_events();

    println!("{:#?}", events);
  
    // TODO broken
    //db.drop_cal("test".to_owned()).unwrap();

    println!("{:?}", db.get_cals());
}
