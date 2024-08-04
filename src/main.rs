use axum::{
  extract::Path,
  http::StatusCode,
  routing::{get, post},
  Json, Router,
};
use serde_json::{to_string_pretty, Value};
use std::{
  fs::File,
  io::Write,
  path::PathBuf,
  time::{SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new().route("/", get(home)).route("/data/:name", post(save_data));
  // read port from environment variable, defaults to 3000
  let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
  println!("Listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn home() -> (StatusCode, String) {
  (
    StatusCode::OK,
    "this is a home page of data backs. pass data to /data/:name with JSON body to save it".to_owned(),
  )
}

async fn save_data(
  // this argument tells axum to parse the request body
  Path(name): Path<String>,
  // as JSON into a `Data` type
  Json(payload): Json<Value>,
) -> (StatusCode, String) {
  let data = to_string_pretty(&payload).unwrap();
  println!("Data received for {}: {}", name, data);

  let filename = generate_filename();
  let path = PathBuf::from(format!("data/{}", filename));

  // Create directory if it doesn't exist
  if !path.parent().unwrap().exists() {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
  }

  let mut file = File::create(path).unwrap();
  file.write_all(data.as_bytes()).unwrap();

  println!("Data saved to {}", filename);

  (StatusCode::OK, "Data saved successfully".to_string())
}

// Generates a filename with date in the format YYYY-MM-DD.json
fn generate_filename() -> String {
  let now = SystemTime::now();
  let duration = now.duration_since(UNIX_EPOCH).unwrap();
  let seconds = duration.as_secs();

  let date = chrono::DateTime::from_timestamp(seconds as i64, 0).expect("Invalid timestamp");

  format!("{}.json", date.format("%Y-%m-%d"))
}
