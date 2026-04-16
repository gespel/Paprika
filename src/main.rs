mod requester;
use colored::Colorize;
use std::{io::Write};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use requester::health_check::HealthChecker;
use requester::model_request::ModelRequester;

struct Paprika {
    health_checker_handle: tokio::task::JoinHandle<()>,
    model_requester: ModelRequester
}

impl Paprika {
    pub fn new() -> Self {
        Paprika { 
            health_checker_handle: HealthChecker::create_health_checker_handle(5000),
            model_requester: ModelRequester::new("gemma4:e4b")
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

    let mut p = Paprika::new();
    let health_handle = p.health_checker_handle;

    let question: &str = "Why is the sky blue? Answer as brief as possible";

    let r = p.model_requester.request_full_text(question).await;
    println!("Question: {}\nAnswer from LLM: {}", question, r);

    let _ = health_handle.await;
}