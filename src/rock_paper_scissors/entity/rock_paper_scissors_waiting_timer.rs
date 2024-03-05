use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct RockPaperScissorsWaitingTimer {
    timers: HashMap<i32, Instant>,
}

impl RockPaperScissorsWaitingTimer {
    pub fn new() -> RockPaperScissorsWaitingTimer {
        RockPaperScissorsWaitingTimer {
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