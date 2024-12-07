
use crate::fetcher;
use crate::parser;
use crate::storage;
use crate::config::Config;
use std::collections::{HashSet, VecDeque};
use tokio::sync::Mutex;
use std::sync::Arc;

/// Starts the crawling process using the provided configuration.
pub async fn start(config: Config) {
    let visited_urls = Arc::new(Mutex::new(HashSet::new()));
    let url_queue = Arc::new(Mutex::new(VecDeque::from(vec![config.start_url.clone()])));

    crawl(url_queue, visited_urls, config.depth_limit).await;
}

/// The main crawling loop that processes URLs asynchronously up to a specified depth.
async fn crawl(url_queue: Arc<Mutex<VecDeque<String>>>, visited_urls: Arc<Mutex<HashSet<String>>>, depth_limit: usize) {
    while let Some(url) = get_next_url(&url_queue, &visited_urls).await {
        match fetcher::fetch_page(&url).await {
            Ok(html) => {
                println!("Fetched URL: {}", url);
                let links = parser::extract_links(&url, &html);
                storage::save_to_csv(&url, &html).unwrap();
                add_new_links_to_queue(links, &url_queue, &visited_urls, depth_limit).await;
            }
            Err(e) => eprintln!("Error fetching {}: {}", url, e),
        }
    }
}

/// Gets the next URL from the queue, ensuring it hasn't been visited.
async fn get_next_url(url_queue: &Arc<Mutex<VecDeque<String>>>, visited_urls: &Arc<Mutex<HashSet<String>>>) -> Option<String> {
    let mut queue = url_queue.lock().await;
    let mut visited = visited_urls.lock().await;

    while let Some(url) = queue.pop_front() {
        if !visited.contains(&url) {
            visited.insert(url.clone());
            return Some(url);
        }
    }
    None
}

/// Adds new links to the queue if they haven't been visited and are within the depth limit.
async fn add_new_links_to_queue(links: Vec<String>, url_queue: &Arc<Mutex<VecDeque<String>>>, visited_urls: &Arc<Mutex<HashSet<String>>>, depth_limit: usize) {
    let mut queue = url_queue.lock().await;
    let visited = visited_urls.lock().await;

    for link in links {
        if !visited.contains(&link) && queue.len() < depth_limit {
            queue.push_back(link);
        }
    }
}
