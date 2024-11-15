
// Purpose: create a crawler app that could access the network language datasets. 
// Date: 2024/11/15

mod crawler;
mod config;

fn main() {
    let config = config::Config::from_args();
    crawler::start(config);
}