use std::os::unix::thread;
use reqwest::Response;
use tokio::time::{sleep, Duration};

pub struct HealthChecker {
    check_intervall: u32
}

impl HealthChecker {
    pub fn new(check_intervall: u32) -> Self {
        HealthChecker {
            check_intervall
        }
    }

    pub async fn check_health(&mut self) -> bool {
        let r = reqwest::get("http://127.0.0.1:11434").await;
        
        match r {
            Ok(r) => {
                let t = r.text().await;
                
                match t {
                    Ok(text) => {
                        if text == "Ollama is running" {
                            true
                        }
                        else {
                            println!("Unexpected message from ollama!");
                            false
                        }
                    }
                    Err(e) => {
                        println!("Could not parse response string from ollama! {:?}", e);
                        false
                    }
                }
            },
            Err(e) => {
                println!("Ollama did not respond! {:?}", e);
                false
            }
        }
    }

    pub fn create_health_checker_handle(check_intervall: u32) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut hc = HealthChecker::new(check_intervall);

            loop {
                if hc.check_health().await == true {
                    println!("Ollama is available!");
                }
                else {
                    println!("Error. Backend service is not available!");
                }
                sleep(Duration::from_millis(check_intervall as u64)).await;
            }
        })
    }
}

