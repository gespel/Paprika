mod requester;

use requester::health_check::HealthChecker;

struct Paprika {
    health_checker_handle: tokio::task::JoinHandle<()>
}

impl Paprika {
    pub fn new() -> Self {
        Paprika { 
            health_checker_handle: HealthChecker::create_health_checker_handle(1000)
        }
    }
}

#[tokio::main]
async fn main() {
    let p = Paprika::new();
    loop {
        
    }
}