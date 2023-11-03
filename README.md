[![Rust CI/CD](https://github.com/nogibjj/lisa-mini-project-8/actions/workflows/rust-cicd.yml/badge.svg)](https://github.com/nogibjj/lisa-mini-project-8/actions/workflows/rust-cicd.yml)

## Rust-Powered Campus Seminar Information Management

**Project Overview:**
The project aims to create a Rust-based CLI application that scrapes seminar information from a campus department website, stores the scraped data in an SQLite database, allows users to query the database based on dates, and optimizes the Rust binary. Additionally, it incorporates GitHub Actions to ensure continuous integration and delivery, including testing, building, linting, and generating optimized Rust binaries that can be easily downloaded. Below are the detailed steps to achieve this:


### User Guide

**Dependencies**
1. Rust: Ensure you have Rust and Cargo installed on your system, which can be done by following the official Rust installation guide (https://www.rust-lang.org/).

2. SQLite: The project relies on SQLite as the database for storing the scraped event data. SQLite is a self-contained, serverless, and zero-configuration SQL database engine that you should have installed or use a library to interact with it in your Rust project.

**How to Run the Program**

1. Clone the project repository from GitHub:

   ```bash
   https://github.com/lisawym/Lisa-Individual-Project-2.git
   ```

2. Build the Rust CLI binary:

   ```bash
   cargo build --release
   ```

   This will generate an optimized binary in the "target/release" directory.

3. Run the Rust CLI binary to scrape event data and store it in the SQLite database:

   ```bash
   target/release/event_scraper
   ```



### Approach to the Problem
**Step 1: Web Scraping (Rust Source Code)**
- Utilize Rust to build a web scraping tool to extract seminar information from the campus department website.
- Comprehensively understand Rust's syntax and unique features.

**Step 2: Database Operations**
- Store the scraped seminar data into an SQLite database.
- Demonstrate CRUD (Create, Read, Update, Delete) operations on the database.

**Step 3: Database Query**
- Create functionality for users to query the database based on specified dates.

**Step 4: GitHub Actions Workflow**
- Implement GitHub Actions to automate various processes.
- Include testing, linting, and building Rust code as part of the workflow.

**Step 5: Binary Optimization**
- Optimize the Rust binary to enhance its performance and efficiency.



**GitHub Copilot Usage**
GitHub Copilot can assist in the coding process in various ways:
- It can generate code snippets for common tasks in Rust, such as working with SQLite, handling HTTP requests, and processing data.
- Copilot can provide suggestions and auto-complete code based on your existing code and Rust best practices.
- Create a detailed README.md that explains the project's purpose, dependencies, how to run the program, and how GitHub Copilot was used in the coding process.
