use axum::{
    routing::{get, post},
    extract::State,
    response::{Html, Json},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/counter", get(show_counter))
        .route("/increment", post(increment))
        .route("/decrement", post(decrement))
        .route("/reset", post(reset))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running at http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

// 🟢 Page 1: "Hello, World!"
async fn hello_world() -> Html<String> {
    Html("<h1>Hello, World!</h1><p><a href='/static/index.html'>Go to Counter</a></p>".to_string())
}

// 🔢 Show counter
async fn show_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let counter = state.counter.lock().unwrap();
    Json(CounterResponse { value: *counter })
}

// ➕ Increment
async fn increment(State(state): State<AppState>) -> Json<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    Json(CounterResponse { value: *counter })
}

// ➖ Decrement
async fn decrement(State(state): State<AppState>) -> Json<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter -= 1;
    Json(CounterResponse { value: *counter })
}

// 🔄 Reset
async fn reset(State(state): State<AppState>) -> Json<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter = 0;
    Json(CounterResponse { value: *counter })
}

#[derive(Serialize, Deserialize)]
struct CounterResponse {
    value: i32,
}
