mod requester;
mod server;
use colored::Colorize;
use std::{io::Write};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use requester::health_check::HealthChecker;
use requester::model_request::ModelRequester;
use std::io;
use std::io::*;

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
    let _ = p.health_checker_handle;
    
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let r = p.model_requester.request_full_text(input.as_str()).await;
        println!("{}", r.green());
    }
}