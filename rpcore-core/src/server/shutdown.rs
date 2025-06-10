use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub trait Shutdown {
    fn shutdown(&self);
}

pub trait IsShuttingDown {
    fn is_shutting_down(&self) -> bool;
}

#[derive(Default, Clone)]
pub struct ShutdownBool {
    bool: Arc<AtomicBool>,
}

impl ShutdownBool {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Shutdown for ShutdownBool {
    fn shutdown(&self) {
        self.bool.store(true, Ordering::Release);
    }
}

impl IsShuttingDown for ShutdownBool {
    fn is_shutting_down(&self) -> bool {
        self.bool.load(Ordering::Acquire)
    }
}
