use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

#[derive(Clone)]
pub struct ConcurrencyGate {
    semaphore: Arc<Semaphore>,
}

impl ConcurrencyGate {
    pub fn new(max_permits: usize) -> Self {
        let max = if max_permits == 0 { 1 } else { max_permits };
        Self {
            semaphore: Arc::new(Semaphore::new(max)),
        }
    }

    pub fn from_env() -> Self {
        let max: usize = std::env::var("LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        Self::new(max)
    }

    pub async fn acquire(&self) -> OwnedSemaphorePermit {
        self.semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed")
    }
}
