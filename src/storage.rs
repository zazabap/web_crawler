use std::fs::File;
use std::io::{Write, Error as IoError};
use csv::Writer;
use rusqlite::{params, Connection, Result as SqlResult};

/// Saves a row of data to a CSV file.
pub fn save_to_csv(url: &str, html_content: &str) -> Result<(), IoError> {
    let file = File::options().append(true).create(true).open("output.csv")?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&[url, html_content])?;
    wtr.flush()?;
    Ok(())
}

/// Saves crawled data into an SQLite database.
pub fn save_to_sqlite(url: &str, html_content: &str) -> SqlResult<()> {
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

