use scraper::{Html, Selector};
use url::Url;

/// Extracts links from the HTML content, returning a vector of absolute URLs as strings.
pub fn extract_links(base_url: &str, html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap(); // Select all <a> tags with an href attribute
    let base_url = Url::parse(base_url).expect("Invalid base URL");

    document.select(&selector)
        .filter_map(|element| element.value().attr("href")) // Extract href attribute
        .filter_map(|href| resolve_url(&base_url, href))    // Convert to absolute URL
        .collect()
}

/// Resolves a potentially relative URL to an absolute URL, based on the base URL.
fn resolve_url(base_url: &Url, href: &str) -> Option<String> {
    match base_url.join(href) {
        Ok(url) => Some(url.to_string()),
        Err(_) => None, // Ignore invalid URLs
    }
}
