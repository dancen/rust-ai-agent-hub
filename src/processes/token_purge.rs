use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use crate::services::agent_service::AgentService;

pub fn start_token_purge_task(service: Arc<AgentService>, interval_seconds: u64) {
    tokio::spawn(async move {
        loop {
            match service.purge_tokens().await {
                Ok(count) => {
                    println!("[TokenPurge] Removed {} expired tokens", count);
                }
                Err(err) => {
                    eprintln!("[TokenPurge] Error while cleaning up tokens: {:?}", err);
                }
            }

            sleep(Duration::from_secs(interval_seconds)).await;
        }
    });
}
