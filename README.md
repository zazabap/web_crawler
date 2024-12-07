# Rust Web Crawler

A simple, asynchronous web crawler written in Rust. This project demonstrates core web crawling functionality, including fetching web pages, parsing links, handling URL normalization, and storing crawled data. The crawler can be configured to start from any URL and is capable of limiting the crawl to a specific domain or depth.

## Features

- **Asynchronous Crawling**: Efficiently fetches multiple pages in parallel.
- **Configurable Depth and Page Limits**: Control how deep the crawler goes and the maximum number of pages to crawl.
- **Domain Restriction**: Optionally restricts crawling to URLs within the same domain as the start URL.
- **Data Storage**: Saves crawled data to both CSV and SQLite for easy data analysis.
- **URL Validation and Normalization**: Handles URL processing to avoid duplicate or invalid links.
- **Visualization**: Provides a tool to view crawled data directly from the SQLite database.

---

## Installation

### Prerequisites
- **Rust**: Install Rust by following instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Dependencies**: Run `cargo build` to install necessary dependencies, including `reqwest`, `scraper`, `csv`, and `rusqlite`.

### Setting Up

1. Clone this repository:
    ```bash
    git clone 
    cd rust-web-crawler
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

---

## Usage

Run the crawler from the command line, providing the necessary arguments:

```bash
cargo run -- --start-url "https://example.com" --depth-limit 3 --max-pages 100 --same-domain
```

### Arguments:
- `--start-url`: (Required) The starting URL for the crawler.
- `--depth-limit`: (Optional) Maximum depth of the crawl. Defaults to `3`.
- `--max-pages`: (Optional) Maximum number of pages to crawl.
- `--same-domain`: (Optional) Restrict crawling to the starting domain.

---

## Visualizing the Output

### Viewing Data in SQLite
1. Install a tool like **DB Browser for SQLite**:
   - [Download DB Browser for SQLite](https://sqlitebrowser.org/).
   - Open the `output.db` file to browse the crawled data.

2. Or use the SQLite CLI:
   ```bash
   sqlite3 output.db
   ```

   Query data:
   ```sql
   SELECT * FROM crawled_data;
   ```

### Viewing Data in CSV
1. Open `output.csv` in a spreadsheet application like Microsoft Excel or LibreOffice Calc.
2. Analyze the rows of URLs and their corresponding HTML content.

---

## Developer Tool: Data Visualization
Use the included visualization tool to display crawled data in a tabular format via the terminal:

1. Build and run the visualization binary:
   ```bash
   cargo run --bin visualize
   ```

2. Output example:
   ```
   ID    URL                                              HTML Content (truncated)
   --------------------------------------------------------------------------------
   1     http://example.com                               <html>Example</html>
   2     http://example.com/page2                         <html>Page 2</html>
   ```

---

## Contributions

Contributions are welcome! Feel free to submit issues, feature requests, or pull requests to improve this project.
