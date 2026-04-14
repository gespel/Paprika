mod requester;
use colored::Colorize;
use core::time;
use std::{io::Write, thread};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use requester::health_check::HealthChecker;

struct Paprika {
    health_checker_handle: tokio::task::JoinHandle<()>
}

impl Paprika {
    pub fn new() -> Self {
        Paprika { 
            health_checker_handle: HealthChecker::create_health_checker_handle(5000)
        }
    }
}

fn setup_logging() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "[{}] {} [{}] - {}",
                "Paprika".red(),
                Local::now().format("%Y-%m-%dT%H:%M:%S").to_string().blue(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logging();

    let p = Paprika::new();
    p.health_checker_handle.await;
}