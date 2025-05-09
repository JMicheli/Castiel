mod discovery;

use axum::{Router, routing::post};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  let app = Router::new()
    .route("/scan", post(scan))
    .fallback_service(serve_dir);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  println!("Listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn scan() {
  println!("Attempting chromecast scan!");
  let discover_output = discovery::find_chromecasts(1);

  println!("Resolved discover output: {discover_output:?}")
}
