use url::{Url, ParseError};

/// Checks if a given URL string is valid.
pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

/// Normalizes a URL by ensuring it has a standard format (e.g., https://example.com).
pub fn normalize_url(url: &str) -> Option<String> {
    match Url::parse(url) {
        Ok(parsed_url) => Some(parsed_url.to_string()),
        Err(_) => None,
    }
}

/// Checks if a URL belongs to the same domain as the base URL.
/// Returns `true` if the domains match, otherwise `false`.
pub fn is_same_domain(base_url: &str, target_url: &str) -> bool {
    if let (Ok(base), Ok(target)) = (Url::parse(base_url), Url::parse(target_url)) {
        base.domain() == target.domain()
    } else {
        false
    }
}
