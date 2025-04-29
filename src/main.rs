


use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::fs;



async fn serve_login_page() -> Html<String> {
    // Read the login.html file
    let html_content = fs::read_to_string("target/static/index.html")
        .expect("Could not read the login.html file");
    
    Html(html_content)
}

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(serve_login_page))  
        .route("/profile", get(|| async { "man why are you here are you gay? " }));  

    // run our app with hyper, listening globally on port 3000
    println!("server is running on the localhost");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}