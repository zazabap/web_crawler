use std::fs::File;
use std::io::{Write, Error as IoError};
use csv::Writer;
use rusqlite::{params, Connection, Result as SqlResult};

/// Enum representing storage backends.
pub enum StorageBackend {
    Csv,
    Sqlite,
}

/// Struct to manage the storage configuration.
pub struct StorageManager {
    backend: StorageBackend,
}

impl StorageManager {
    /// Creates a new `StorageManager` with the specified backend.
    pub fn new(backend: StorageBackend) -> Self {
        StorageManager { backend }
    }

    /// Saves a URL and its HTML content using the selected storage backend.
    pub fn save(&self, url: &str, html_content: &str) -> Result<(), String> {
        match self.backend {
            StorageBackend::Csv => {
                save_to_csv(url, html_content).map_err(|e| format!("CSV Error: {}", e))
            }
            StorageBackend::Sqlite => {
                save_to_sqlite(url, html_content).map_err(|e| format!("SQLite Error: {}", e))
            }
        }
    }
}

/// Saves a row of data to a CSV file.
fn save_to_csv(url: &str, html_content: &str) -> Result<(), IoError> {
    let file = File::options().append(true).create(true).open("output.csv")?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&[url, html_content])?;
    wtr.flush()?;
    Ok(())
}

/// Saves crawled data into an SQLite database.
fn save_to_sqlite(url: &str, html_content: &str) -> SqlResult<()> {
    let conn = Connection::open("output.db")?;

    // Ensure the table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS crawled_data (
            id INTEGER PRIMARY KEY,
            url TEXT UNIQUE,
            html_content TEXT
        )",
        [],
    )?;

    // Insert the data
    conn.execute(
        "INSERT OR IGNORE INTO crawled_data (url, html_content) VALUES (?1, ?2)",
        params![url, html_content],
    )?;

    Ok(())
}
