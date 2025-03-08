use std::collections::{HashSet, VecDeque};
use tokio::sync::Mutex;
use std::sync::Arc;
use url::Url;
use scraper::{Html, Selector};
use crate::config::Config;

#[derive(Debug)]
pub struct Page {
    pub url: String,
    pub title: Option<String>,
    pub status_code: Option<i32>,
}

pub async fn crawl(config: &Config) -> Result<Vec<Page>, Box<dyn std::error::Error + Send + Sync>> {
    let visited_urls: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    let url_queue = Arc::new(Mutex::new(VecDeque::from(vec![(config.start_url.clone(), 0)])));
    let results = Arc::new(Mutex::new(Vec::new()));
    let base_domain = if config.same_domain {
        extract_domain(&config.start_url)?
    } else {
        String::new()
    };

    while let Some((url, depth)) = get_next_url(&url_queue, &visited_urls).await {
        if depth >= config.depth_limit {
            continue;
        }

        let results_clone = Arc::clone(&results);
        let queue_clone = Arc::clone(&url_queue);
        let visited_clone = Arc::clone(&visited_urls);
        
        match fetch_page(&url).await {
            Ok((title, status, links)) => {
                // Process the page
                let mut results = results_clone.lock().await;
                results.push(Page {
                    url: url.clone(),
                    title,
                    status_code: Some(status),
                });

                // Add new links to the queue (in a separate step to avoid holding the results lock)
                for link in links {
                    if should_crawl_url(&link, &base_domain, config.same_domain) {
                        let mut queue = queue_clone.lock().await;
                        let visited = visited_clone.lock().await;
                        if !visited.contains(&link) {
                            queue.push_back((link, depth + 1));
                        }
                    }
                }

                if let Some(max_pages) = config.max_pages {
                    if results.len() >= max_pages {
                        break;
                    }
                }
            }
            Err(e) => eprintln!("Error processing {}: {}", url, e),
        }
    }

    let final_results = Arc::try_unwrap(results)
        .unwrap()
        .into_inner();

    Ok(final_results)
}

// Fetch a page and return the title, status code, and extracted links
async fn fetch_page(
    url: &str,
) -> Result<(Option<String>, i32, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let status = response.status().as_u16() as i32;
    let html = response.text().await?;
    
    // Process HTML in a more Send-friendly way
    let document = Html::parse_document(&html);
    
    // Extract title
    let title = document
        .select(&Selector::parse("title").unwrap())
        .next()
        .and_then(|title| Some(title.inner_html()));
    
    // Extract links (do this before we return from the function)
    let links = extract_links(url, &document);
    
    Ok((title, status, links))
}

fn extract_domain(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let parsed = Url::parse(url)?;
    Ok(parsed.host_str().unwrap_or("").to_string())
}

fn should_crawl_url(url: &str, base_domain: &str, same_domain: bool) -> bool {
    if !same_domain {
        return true;
    }

    if let Ok(parsed) = Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host.contains(base_domain);
        }
    }
    false
}

async fn get_next_url(
    url_queue: &Arc<Mutex<VecDeque<(String, usize)>>>,
    visited_urls: &Arc<Mutex<HashSet<String>>>,
) -> Option<(String, usize)> {
    let mut queue = url_queue.lock().await;
    let mut visited = visited_urls.lock().await;

    while let Some((url, depth)) = queue.pop_front() {
        if !visited.contains(&url) {
            visited.insert(url.clone());
            return Some((url, depth));
        }
    }
    None
}

fn extract_links(base_url: &str, document: &Html) -> Vec<String> {
    let selector = Selector::parse("a[href]").unwrap();
    let base_url = Url::parse(base_url).unwrap();

    document
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            base_url.join(href).ok().map(|url| url.to_string())
        })
        .collect()
}
