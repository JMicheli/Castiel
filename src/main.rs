use axum::{Router, routing::post};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  let app = Router::new()
    .route("/press", post(press))
    .fallback_service(serve_dir);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn press() {
  println!("Button pressed!")
}
