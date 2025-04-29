use axum::{
    routing::get,
    Router,
    response::Html,
    http::{HeaderMap, HeaderValue},
    middleware::{self, Next},
};
use std::{fs, env};
use std::path::Path;
use axum::response::Response;
use axum::extract::Request;

// CORS middleware
async fn add_cors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    let mut modified_response = response;
    
    let headers = modified_response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, POST, PUT, DELETE"));
    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type"));
    
    modified_response
}

async fn serve_login_page() -> Html<String> {
    // Determine the correct path for static files
    let static_path = if Path::new("target/static/index.html").exists() {
        "target/static/index.html"
    } else {
        // Fallback path for production
        "static/index.html"
    };
    
    // Read the HTML file
    let html_content = fs::read_to_string(static_path)
        .expect("Could not read the index.html file");
    
    Html(html_content)
}

#[tokio::main]
async fn main() {
    // Get port from environment variable (Render sets this)
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    // Build our application with routes and middleware
    let app = Router::new()
        .route("/", get(serve_login_page))
        .route("/profile", get(|| async { "man why are you here are you gay? " }))
        .layer(middleware::from_fn(add_cors));
    
    println!("Server is running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
