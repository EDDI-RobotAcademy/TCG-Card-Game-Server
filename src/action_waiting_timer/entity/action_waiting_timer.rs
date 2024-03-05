use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct ActionWaitingTimer {
    timers: HashMap<i32, Instant>,
}

impl ActionWaitingTimer {
    pub fn new() -> ActionWaitingTimer {
        ActionWaitingTimer {
            timers: HashMap::new(),
        }
    }

    pub fn start_timer(&mut self, id: i32) {
        self.timers.insert(id, Instant::now());
    }
    pub fn check_timer(&self, id: i32, duration: Duration) -> bool {
        if let Some(&start_time) = self.timers.get(&id) {
            let elapsed_time = Instant::now() - start_time;
            elapsed_time >= duration
        } else {
            false
        }
    }
}
