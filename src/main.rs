use axum::{
    routing::{get, post},
    extract::{State, Json},
    response::{Html, Json as JsonResponse},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    output: String,
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
        .route("/run", post(run_rust_code))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running at http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

// 🟢 Page 1: "Hello, World!"
async fn hello_world() -> Html<String> {
    Html("<h1>Hello, World!</h1><p><a href='/static/index.html'>Go to Counter</a></p><p><a href='/static/console.html'>Run Rust Program</a></p>".to_string())
}

// 🔢 Show counter
async fn show_counter(State(state): State<AppState>) -> JsonResponse<CounterResponse> {
    let counter = state.counter.lock().unwrap();
    JsonResponse(CounterResponse { value: *counter })
}

// ➕ Increment
async fn increment(State(state): State<AppState>) -> JsonResponse<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    JsonResponse(CounterResponse { value: *counter })
}

// ➖ Decrement
async fn decrement(State(state): State<AppState>) -> JsonResponse<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter -= 1;
    JsonResponse(CounterResponse { value: *counter })
}

// 🔄 Reset
async fn reset(State(state): State<AppState>) -> JsonResponse<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter = 0;
    JsonResponse(CounterResponse { value: *counter })
}

// 🏃 Run Rust code
async fn run_rust_code(Json(payload): Json<RunRequest>) -> JsonResponse<RunResponse> {
    let code = payload.code;
    let mut child = Command::new("rustc")
        .arg("-")
        .arg("-o")
        .arg("/tmp/temp_program")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to compile Rust code");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(code.as_bytes()).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    if !output.status.success() {
        return JsonResponse(RunResponse {
            output: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let output = Command::new("/tmp/temp_program")
        .output()
        .expect("Failed to run Rust program");

    JsonResponse(RunResponse {
        output: String::from_utf8_lossy(&output.stdout).to_string(),
    })
}

#[derive(Serialize, Deserialize)]
struct CounterResponse {
    value: i32,
}
