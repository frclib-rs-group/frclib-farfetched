#![feature(thread_local)]
pub mod measuring;
pub mod system;
pub mod types;
mod webpage;

use std::process::Command;

use measuring::{measure_processes, measure_stats};

use axum::{
    routing::{get, post},
    Json, Router,
};
use crate::types::{routes, Processes, Stats, Summary};
use static_init::dynamic;
use sysinfo::{System, SystemExt};

#[cfg(target_vendor = "roborio")]
pub mod rio_interface;
#[cfg(target_vendor = "roborio")]
pub mod ini;

#[cfg(not(target_os = "linux"))]
compile_error!("This program is only supported on Linux");

#[dynamic]
#[thread_local]
static mut SYSTEM: System = System::new_all();
#[dynamic]
static SUMMARY: Summary = system::make_summary();

macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::info!($($arg)*);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(feature = "logging")]
    {
        tracing_subscriber::fmt::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    }

    // let addr = std::net::SocketAddr::from(([10, 64, 60, 53], 80));
    //create addr of local host and port 80
    let localhost_addr = std::net::SocketAddr::from(([127, 0, 0, 1], 80));

    let app = Router::new()
        .route(routes::ROOT, get(root))
        .route(routes::STATS, get(all_stats))
        .route(routes::PROCESSES, get(processes))
        .route(routes::SYSTEM_SUMMARY, get(system_summary))
        .route(routes::TIME, get(get_time))
        .route(routes::TIME, post(set_time))
        .route(routes::REBOOT, post(reboot))
        .route(routes::UPTIME, get(get_uptime))
        .route(routes::SET_IP, post(set_static_ip))
        ;

    thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Min).unwrap();

    info!("Router made, starting server");

    axum::Server::bind(&localhost_addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

async fn root() -> webpage::Webpage {
    webpage::Webpage
}

use thiserror::Error;

async fn all_stats() -> Json<Stats> {
    Json(measure_stats(&mut SYSTEM.write()))
}

async fn processes() -> Json<Processes> {
    Json(measure_processes(&mut SYSTEM.write()))
}

async fn system_summary() -> Json<Summary> {
    Json(SUMMARY.clone())
}

async fn get_time() -> String {
    use crate::types::timespec_to_hex;

    let time_spec = nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME).unwrap();
    //if pointer width is 32, then the timespec is 32 bits, otherwise it's 64
    #[cfg(target_pointer_width = "32")]
    {
        return timespec_to_hex(time_spec.tv_sec() as i64, time_spec.tv_nsec() as i64);
    }
    #[cfg(target_pointer_width = "64")]
    {
        return timespec_to_hex(time_spec.tv_sec(), time_spec.tv_nsec());
    }
}

async fn set_time(hex: String) -> &'static str {
    use crate::types::hex_to_timespec;

    let decoded_timespec = hex_to_timespec(hex);
    #[cfg(target_pointer_width = "32")]
    {
        nix::time::clock_settime(
            nix::time::ClockId::CLOCK_REALTIME,
            nix::sys::time::TimeSpec::new(decoded_timespec.0 as i32, decoded_timespec.1 as i32),
        )
        .unwrap();
    }
    #[cfg(target_pointer_width = "64")]
    {
        nix::time::clock_settime(
            nix::time::ClockId::CLOCK_REALTIME,
            nix::sys::time::TimeSpec::new(decoded_timespec.0, decoded_timespec.1),
        )
        .unwrap();
    }
    "Time set"
}

async fn get_uptime() -> String {
    crate::types::timespec_to_hex(SYSTEM.read().uptime() as i64, 0)
}

async fn reboot(verification: String) -> &'static str {
    if verification != crate::types::REBOOT_VERIFICATION {
        return "Verification string incorrect";
    }
    nix::sys::reboot::reboot(nix::sys::reboot::RebootMode::RB_AUTOBOOT).unwrap();
    "Rebooting"
}

#[derive(serde::Deserialize)]
struct StaticIpConfig {
    interface: String,
    ip: String,
    gateway: String,
}

async fn set_static_ip(Json(config): Json<StaticIpConfig>) -> &'static str {
    #[cfg(target_vendor = "roborio")]
    {
        rio_interface::write_static_ip(config.ip.clone(), config.gateway.clone(), config.gateway);
    }
    Command::new("ip")
        .args(&["addr", "add", &format!("{}/24", config.ip), "dev", &config.interface])
        .output()
        .expect("Failed to set IP");
    "Static IP set"
}

#[derive(Debug, Error)]
enum ShiitakeError {
    #[error("Failed to read file")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Failed to parse float")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Data not found")]
    DataNotFound,
}
