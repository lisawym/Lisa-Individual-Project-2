// define a custom data structure
// to store the scraped data
#[derive(Debug)]
struct Event {
    title: Option<String>,
    organizer: Option<String>,
    category: Option<String>,
    time: Option<String>,
    location: Option<String>,
    link: Option<String>,
    series: Option<String>,
    speaker: Option<String>,
    detail: Option<String>,
}

fn scrape_data() {
    let url = "https://math.duke.edu/events";
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();
    let document = scraper::Html::parse_document(&body);

    let html_product_selector = scraper::Selector::parse(".views-row").unwrap();
    let html_products = document.select(&html_product_selector);

    // initialize the vector that will store the scraped data
    let mut events: Vec<Event> = Vec::new();

    for html_product in html_products {
        let link = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let title = html_product
            .select(&scraper::Selector::parse(".views-field-field-display-title").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let time = html_product
            .select(&scraper::Selector::parse(".views-field-field-event-date").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let location = html_product
            .select(&scraper::Selector::parse(".views-field-field-event-location").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let series = html_product
            .select(&scraper::Selector::parse(".event-series").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let speaker = html_product
            .select(&scraper::Selector::parse(".views-field-field-event-speakers").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let detail = html_product
            .select(&scraper::Selector::parse(".views-field-nothing").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let category = "Seminar";
        let organizer = "Duke Mathematics Department";

        if let Some(ref value) = title {
            // instanciate a new event product
            // with the scraped data and add it to the list
            let event = Event {
                title,
                organizer: Some(organizer.to_string()),
                category: Some(category.to_string()),
                time,
                location,
                link,
                series,
                speaker,
                detail,
            };
            events.push(event);
        }
    }

    // print the list of products
    println!("{:#?}", events);

    /* connect to a sqlite database and write events in a event table */
    let conn = rusqlite::Connection::open("events.db").unwrap();
    conn.execute(  
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            organizer TEXT NOT NULL,
            category TEXT NOT NULL,
            time TEXT NOT NULL,
            location TEXT NOT NULL,
            link TEXT NOT NULL,
            series TEXT,
            speaker TEXT,
            detail TEXT
        )",
        [],
    )
    .unwrap();
}

fn main() {
    scrape_data();
}
