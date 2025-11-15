use std::{
    fmt,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy)]
pub enum Phase {
    Starting,
    Ready,
    Errored,
    ShuttingDown,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Phase::Starting => "starting",
            Phase::Ready => "ready",
            Phase::Errored => "errored",
            Phase::ShuttingDown => "shutting_down",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
struct BotStatus {
    phase: Phase,
    last_error: Option<String>,
    started_at: Instant,
}

#[derive(Debug, Clone)]
pub struct BotStatusSnapshot {
    pub phase: Phase,
    pub last_error: Option<String>,
    pub uptime: Duration,
}

impl BotStatusSnapshot {
    pub fn uptime_human(&self) -> String {
        let secs = self.uptime.as_secs();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;

        if h > 0 {
            format!("{h}h {m}m {s}s")
        } else if m > 0 {
            format!("{m}m {s}s")
        } else {
            format!("{s}s")
        }
    }
}

#[derive(Clone)]
pub struct StatusManager {
    inner: Arc<RwLock<BotStatus>>,
}

impl StatusManager {
    fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BotStatus {
                phase: Phase::Starting,
                last_error: None,
                started_at: Instant::now(),
            })),
        }
    }

    pub fn global() -> Self {
        static INSTANCE: OnceLock<StatusManager> = OnceLock::new();
        INSTANCE.get_or_init(StatusManager::new).clone()
    }

    pub async fn set_phase(&self, phase: Phase) {
        let mut guard = self.inner.write().await;
        guard.phase = phase;
    }

    pub async fn set_error<E: Into<String>>(&self, error: E) {
        let mut guard = self.inner.write().await;
        guard.last_error = Some(error.into());
    }

    pub async fn clear_error(&self) {
        let mut guard = self.inner.write().await;
        guard.last_error = None;
    }

    pub async fn snapshot(&self) -> BotStatusSnapshot {
        let guard = self.inner.read().await;
        BotStatusSnapshot {
            phase: guard.phase,
            last_error: guard.last_error.clone(),
            uptime: guard.started_at.elapsed(),
        }
    }
}
