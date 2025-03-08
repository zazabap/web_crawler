// Purpose: create a crawler app that could access the network language datasets. 
// Date: 2024/11/15

mod config;
mod crawler;
mod server;

#[tokio::main]
async fn main() {
    server::start_server().await;
}
