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

    pub async fn check_ollama_health(&mut self) -> bool {
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
                            log::error!("Unexpected message from ollama!");
                            false
                        }
                    }
                    Err(e) => {
                        log::error!("Could not parse response string from ollama! {:?}", e);
                        false
                    }
                }
            },
            Err(e) => {
                log::error!("Ollama did not respond! {:?}", e);
                false
            }
        }
    }

    pub fn create_health_checker_handle(check_intervall: u32) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut hc = HealthChecker::new(check_intervall);

            loop {
                if hc.check_ollama_health().await == true {
                    log::debug!("Ollama online");
                }
                sleep(Duration::from_millis(hc.check_intervall as u64)).await;
            }
        })
    }
}

