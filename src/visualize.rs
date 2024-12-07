use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Connect to the SQLite database
    let conn = Connection::open("output.db")?;

    // Query the crawled data
    let mut stmt = conn.prepare("SELECT id, url, html_content FROM crawled_data")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,      // id
            row.get::<_, String>(1)?,  // url
            row.get::<_, String>(2)?,  // html_content
        ))
    })?;

    // Print the results
    println!("{:<5} {:<50} {:<30}", "ID", "URL", "HTML Content (truncated)");
    println!("{}", "-".repeat(90));

    for row in rows {
        let (id, url, html_content): (i32, String, String) = row?;
        println!(
            "{:<5} {:<50} {:<30}",
            id,
            url,
            &html_content[0..30.min(html_content.len())]  // Truncate content
        );
    }

    Ok(())
}
