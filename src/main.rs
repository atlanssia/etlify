use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_derive::Serialize;
use std::env;
use sysinfo::System;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting...");
    info!("OS: {}", env::consts::OS);
    info!("ARCH: {}", env::consts::ARCH);

    let mut sys = System::new_all();
    sys.refresh_all();

    info!("total memory: {} bytes", sys.total_memory());
    info!("global_cpu_usage: {}", sys.global_cpu_usage());

    // web app
    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(status));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("ok"))
}

async fn status() -> (StatusCode, Json<Status>) {
    let stat = Status {
        status: 0,
        description: String::from("running"),
    };

    (StatusCode::OK, Json(stat))
}

#[derive(Serialize)]
struct Status {
    status: i8,
    description: String,
}
