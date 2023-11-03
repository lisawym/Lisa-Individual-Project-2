

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_data() {
        // Insert some test data into the database
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE events (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                organizer TEXT NOT NULL,
                category TEXT NOT NULL,
                time TEXT NOT NULL,
                location TEXT NOT NULL,
                link TEXT NOT NULL,
                series TEXT NOT NULL,
                speaker TEXT NOT NULL,
                detail TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO events (title, organizer, category, time, location, link, series, speaker, detail)
             VALUES ('Test Event', 'Test Organizer', 'Test Category', '2022-01-01 00:00:00', 'Test Location', 'https://example.com', 'Test Series', 'Test Speaker', 'Test Detail')",
            [],
        )
        .unwrap();

        // Call the query_data function and check the results
        let events = query_data();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, "Test Event");
        assert_eq!(events[0].organizer, "Test Organizer");
        assert_eq!(events[0].category, "Test Category");
        assert_eq!(events[0].time, "2022-01-01 00:00:00");
        assert_eq!(events[0].location, "Test Location");
        assert_eq!(events[0].link, "https://example.com");
        assert_eq!(events[0].series, "Test Series");
        assert_eq!(events[0].speaker, "Test Speaker");
        assert_eq!(events[0].detail, "Test Detail");
    }
}