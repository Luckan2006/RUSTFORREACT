use axum::{routing::get, Router};



#[tokio::main]



async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("10.24.159.214:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    print!("Hej");
}


