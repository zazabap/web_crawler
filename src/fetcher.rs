
use reqwest::Client;
use std::time::Duration;

const TIMEOUT: u64 = 10; // Timeout duration in seconds

/// Fetches the HTML content of a given URL asynchronously.
/// Returns the HTML as a `String` if successful, or an error if the request fails.
pub async fn fetch_page(url: &str) -> Result<String, reqwest::Error> {
    // Create a reqwest client with a timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT))
        .build()?;

    // Perform the GET request
    let response = client.get(url).send().await?;

    // Check if the request was successful (status code 200 OK)
    if response.status().is_success() {
        // Return the HTML content as a String
        Ok(response.text().await?)
    } else {
        // Handle non-200 status codes
        Err(reqwest::Error::new(
            reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(),
            format!("Failed to fetch {}: HTTP {}", url, response.status())
        ))
    }
}

