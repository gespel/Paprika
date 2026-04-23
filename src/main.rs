mod requester;
mod server;
use colored::Colorize;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use requester::health_check::HealthChecker;
use requester::model_request::ModelRequester;
use server::webserver::PaprikaFrontendServer;

#[allow(dead_code)]
struct Paprika {
    health_checker_handle: tokio::task::JoinHandle<()>,
    model_requester: ModelRequester,
    frontend: PaprikaFrontendServer
}

impl Paprika {
    pub fn new() -> Self {
        Paprika { 
            health_checker_handle: HealthChecker::create_health_checker_handle(5000),
            model_requester: ModelRequester::new("gemma4:e4b"),
            frontend: PaprikaFrontendServer::new()
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
    let _ = p.health_checker_handle;
    PaprikaFrontendServer::start().await;
}