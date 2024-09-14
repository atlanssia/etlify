use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{TimeZone, Utc};
use humansize::BINARY;
use serde_derive::Serialize;
use std::{env, time::Duration};
use sysinfo::{Components, Disks, Networks, System};
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

    let formatter = humansize::make_format(BINARY);

    // CPU
    info!("CPUs: {}", sys.cpus().len());
    sys.refresh_cpu_usage();
    info!("global_cpu_usage: {}%", sys.global_cpu_usage());
    let load_avg = System::load_average();
    info!(
        "one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
        load_avg.one, load_avg.five, load_avg.fifteen
    );

    // RAM and swap
    info!("total memory: {}", formatter(sys.total_memory()));
    info!("free memory: {}", formatter(sys.free_memory()));
    info!("used memory: {}", formatter(sys.used_memory()));
    info!("available memory: {}", formatter(sys.available_memory()));

    // Disks
    let disks = Disks::new_with_refreshed_list();
    info!("{disks:?}");

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    info!("{networks:?}");

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    info!("{components:?}");

    // System information
    info!("name: {}", System::name().unwrap());
    info!("host name: {}", System::host_name().unwrap());
    info!("os version: {}", System::long_os_version().unwrap());
    info!("kernel version: {}", System::kernel_version().unwrap());
    info!(
        "uptime: {}",
        humantime::format_duration(Duration::new(System::uptime(), 0))
    );

    let datetime = Utc
        .timestamp_opt(System::boot_time() as i64, 0)
        .unwrap()
        .to_rfc3339();

    info!("boot time: {}", datetime);

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
