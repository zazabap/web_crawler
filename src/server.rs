use axum::{
    routing::{post, get},
    Router,
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use crate::config::Config;
use crate::crawler;

#[derive(Debug, Deserialize)]
pub struct CrawlRequest {
    start_url: String,
    depth_limit: usize,
    max_pages: Option<usize>,
    same_domain: bool,
}

#[derive(Debug, Serialize)]
pub struct CrawlResult {
    url: String,
    title: String,
    status: i32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    status: String,
    version: String,
}

type ApiResponse<T> = Result<(StatusCode, Json<T>), (StatusCode, Json<ErrorResponse>)>;

pub async fn start_server() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/crawl", post(handle_crawl))
        .route("/status", get(handle_status))
        .layer(cors);

    // Run it
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Handler that uses a concrete Result type for responses
async fn handle_crawl(
    Json(payload): Json<CrawlRequest>,
) -> impl IntoResponse {
    tracing::info!("Crawling URL: {}", payload.start_url);
    
    let config = Config {
        start_url: payload.start_url.clone(),
        depth_limit: payload.depth_limit,
        max_pages: payload.max_pages,
        same_domain: payload.same_domain,
    };

    // Use a concrete response type pattern
    let result: ApiResponse<Vec<CrawlResult>> = match crawler::crawl(&config).await {
        Ok(pages) => {
            let results: Vec<CrawlResult> = pages.into_iter()
                .map(|page| CrawlResult {
                    url: page.url,
                    title: page.title.unwrap_or_else(|| "No title".to_string()),
                    status: page.status_code.unwrap_or(0),
                })
                .collect();

            tracing::info!("Crawl completed. Found {} pages.", results.len());
            Ok((StatusCode::OK, Json(results)))
        }
        Err(e) => {
            tracing::error!("Error crawling: {}", e);
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    };

    // Convert the result to a response
    match result {
        Ok(response) => response.into_response(),
        Err(error) => error.into_response(),
    }
}