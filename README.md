# Rust Web Crawler

A modern web crawler with a React frontend and Rust backend. This project demonstrates core web crawling functionality with a user-friendly interface, asynchronous processing, and a RESTful API. The crawler can be configured to start from any URL and is capable of limiting the crawl to a specific domain or depth.

## Features

- **Web Interface**: Clean, responsive React frontend built with Material-UI components.
- **RESTful API**: Backend API built with Axum web framework.
- **Asynchronous Crawling**: Efficiently fetches multiple pages in parallel.
- **Configurable Parameters**: Control depth, page limits, and domain restriction through the UI.
- **Status Indicators**: Color-coded status indicators for crawled pages.
- **Real-time Feedback**: Server status monitoring and error handling.
- **URL Validation**: Frontend and backend validation to ensure proper URLs.
- **Cross-Origin Support**: CORS enabled for API communication.

---

## Installation

### Prerequisites
- **Rust**: Install Rust by following instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Node.js**: Install Node.js and npm from [nodejs.org](https://nodejs.org/).

### Setting Up the Backend

1. Clone this repository:
    ```bash
    git clone https://github.com/zazabap/web_crawler.git
    cd web_crawler
    ```

2. Build the Rust backend:
    ```bash
    cargo build --release
    ```

### Setting Up the Frontend

1. Navigate to the frontend directory:
    ```bash
    cd frontend
    ```

2. Install dependencies:
    ```bash
    npm install
    ```

---

## Usage

### Running the Application

1. Start the backend server:
    ```bash
    cargo run
    ```
    This will start the Rust backend server at http://localhost:8000.

2. In a separate terminal, start the frontend development server:
    ```bash
    cd frontend
    npm run dev
    ```
    This will start the React frontend at http://localhost:5173 (or similar).

3. Open your browser and navigate to the frontend URL.

### Using the Web Interface

1. The interface will automatically check if the backend server is running.
2. Enter a URL to crawl (e.g., https://example.com).
3. Adjust the crawl parameters:
   - **Crawl Depth**: How many links deep to crawl (1-10).
   - **Max Pages**: Maximum number of pages to crawl (10-500).
   - **Stay on Same Domain**: Toggle to restrict crawling to the starting domain.
4. Click "Start Crawling".
5. View the results below, with status codes color-coded:
   - Green: 200-level status (success)
   - Blue: 300-level status (redirect)
   - Yellow: 400-level status (client error)
   - Red: 500-level status (server error)

### API Endpoints

- **GET /status**: Check if the server is running and get version information.
- **POST /crawl**: Start a crawl operation with the following JSON parameters:
  ```json
  {
    "start_url": "https://example.com",
    "depth_limit": 3,
    "max_pages": 100,
    "same_domain": true
  }
  ```

---

## Command Line Usage (Legacy)

The crawler can also be run from the command line:

```bash
cargo run -- --start-url "https://example.com" --depth-limit 3 --max-pages 100 --same-domain
```

### Arguments:
- `--start-url`: (Required) The starting URL for the crawler.
- `--depth-limit`: (Optional) Maximum depth of the crawl. Defaults to `3`.
- `--max-pages`: (Optional) Maximum number of pages to crawl.
- `--same-domain`: (Optional) Restrict crawling to the starting domain.

---

## Storage and Visualization Tools

### Data Storage
The crawler can save data to both SQLite and CSV formats when run from the command line.

### Viewing Stored Data
1. **SQLite**: Use DB Browser for SQLite or the SQLite CLI to view `output.db`.
2. **CSV**: Open `output.csv` in a spreadsheet application.

### Terminal Visualization Tool
For command-line data visualization:

```bash
cargo run --bin visualize
```

---

## Contributions

Contributions are welcome! Feel free to submit issues, feature requests, or pull requests to improve this project.
