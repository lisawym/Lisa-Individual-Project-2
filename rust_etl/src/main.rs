/*
   write a rust data scraping program to get the data from the below address
   Math: https://math.duke.edu/events
   ECE: https://ece.duke.edu/about/events/seminars
   CS: https://cs.duke.edu/events
*/
use std::fs::File;
use std::io::Write;

fn scrape_data() {
    let url = "https://math.duke.edu/events";
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();
    /*
       base on the html structure
       get data in class="views-row row-0 view-teaser-listing first odd
    */
    let document = scraper::Html::parse_document(&body);
    //println!("body = {:?}", document);
    /* instead of li.product, Then, take a look at the HTML code and note that you can get all product elements with this CSS selector: class="views-field views-field-field-event-series" */

    let html_product_selector = scraper::Selector::parse(".views-row").unwrap();
    let html_products = document.select(&html_product_selector);
    //println!("html_products = {:?}", title);

    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
        let title = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        /*
                let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
         */

        println!("title = {:?}", title);
    }
}

fn main() {
    scrape_data();
}
