use crate::event::Event;

pub fn query_data() {
    let conn = rusqlite::Connection::open("events.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM events").unwrap();
    let events_iter = stmt.query_map([], |row| {
        Ok(Event {
            title: row.get(1).unwrap(),
            organizer: row.get(2).unwrap(),
            category: row.get(3).unwrap(),
            time: row.get(4).unwrap(),
            location: row.get(5).unwrap(),
            link: row.get(6).unwrap(),
            series: row.get(7).unwrap(),
            speaker: row.get(8).unwrap(),
            detail: row.get(9).unwrap(),
        })
    }).unwrap();

    //println!("{:#?}", events_iter);
    for event in events_iter {
        match event {
            Ok(event) => println!("Found event{:#?}", event),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}