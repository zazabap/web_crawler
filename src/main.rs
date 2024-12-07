
// Purpose: create a crawler app that could access the network language datasets. 
// Date: 2024/11/15

mod config;
mod fetcher;
mod parser;
mod storage;
mod crawler;

#[tokio::main]
async fn main() {
    let config = config::Config::from_args();
    crawler::start(config).await;
}
