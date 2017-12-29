extern crate core;

use std::sync::atomic::{AtomicU64, Ordering};
use progressbar::IncSignal;

pub struct TestIncCounter {
    value: AtomicU64,
}

impl Default for TestIncCounter {
    fn default() -> TestIncCounter {
        TestIncCounter {
            value: AtomicU64::default(),
        }
    }
}

impl TestIncCounter {
    pub fn value(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl IncSignal for TestIncCounter {
    fn inc(&self, delta: u64) {
        self.value.fetch_add(delta, Ordering::SeqCst);
    }
}
