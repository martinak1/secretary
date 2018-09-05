/// A representation of an event 
#[derive(Debug)]
pub struct Event {
    id: u64,
    name: String,
    desc: String,
    date: String,
}


// functions
impl Event {
    
    /// Create a new event
    pub fn new(name: String, desc: String, date: String) -> Event {
        Event {
            id: 0,
            name,
            desc,
            date
        }
    }


}