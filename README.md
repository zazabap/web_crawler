# Rust Web Crawler

A simple, asynchronous web crawler written in Rust. This project demonstrates core web crawling functionality, including fetching web pages, parsing links, handling URL normalization, and storing crawled data. The crawler can be configured to start from any URL and is capable of limiting the crawl to a specific domain or depth.

## Features

- **Asynchronous Crawling**: Efficiently fetches multiple pages in parallel.
- **Configurable Depth and Page Limits**: Control how deep the crawler goes and the maximum number of pages to crawl.
- **Domain Restriction**: Optionally restricts crawling to URLs within the same domain as the start URL.
- **Data Storage**: Saves crawled data to CSV and SQLite for easy data analysis.
- **URL Validation and Normalization**: Handles URL processing to avoid duplicate or invalid links.

## Installation

### Prerequisites
- **Rust**: Install Rust by following instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Dependencies**: Run `cargo build` to install necessary dependencies, including `reqwest`, `scraper`, `csv`, and `rusqlite`.

### Setting Up

1. Clone this repository:
    ```bash
    git clone <your_repository_url>
    cd rust-web-crawler
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

## Usage

Run the crawler from the command line, providing the necessary arguments:

```bash
cargo run -- --start-url "https://example.com" --depth-limit 3 --max-pages 100 --same-domain
