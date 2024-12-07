use reqwest::Client;
use reqwest::StatusCode;
use std::time::Duration;

const TIMEOUT: u64 = 10; // Timeout duration in seconds

/// Fetches the HTML content of a given URL asynchronously.
/// Returns the HTML as a `String` if successful, or an error if the request fails.
pub async fn fetch_page(url: &str) -> Result<String, String> {
    // Create a reqwest client with a timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Perform the GET request
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Check if the request was successful (status code 200 OK)
    if response.status() == StatusCode::OK {
        // Return the HTML content as a String
        response.text().await.map_err(|e| format!("Failed to read response: {}", e))
    } else {
        Err(format!(
            "Failed to fetch {}: HTTP {}",
            url,
            response.status()
        ))
    }
}
