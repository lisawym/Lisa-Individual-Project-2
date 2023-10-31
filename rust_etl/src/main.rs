use std::env;

mod event;
mod scraper;
mod query;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command: scrape or query");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "scrape" => scraper::main(),
        "query" => query::query_data(),
        _ => println!("Invalid command: {}", command),
    }
}