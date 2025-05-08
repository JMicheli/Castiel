use axum::{
  Router,
  response::Html,
  routing::{get, post},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(index))
    .route("/press", post(press));

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
  Html(include_str!("../static/index.html"))
}

async fn press() {
  println!("Button pressed!")
}
