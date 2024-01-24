use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct MatchWaitingTimer {
    timers: HashMap<i32, Instant>,
}

impl MatchWaitingTimer {
    pub fn new() -> MatchWaitingTimer {
        MatchWaitingTimer {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_5_seconds() {
        let mut timer = MatchWaitingTimer::new();
        let id = 42;
        timer.start_timer(id);

        std::thread::sleep(Duration::from_secs(5));

        assert_eq!(timer.check_timer(id, Duration::from_secs(5)), true);
    }

    #[test]
    fn test_timer_within_5_seconds() {
        let mut timer = MatchWaitingTimer::new();
        let id = 42;
        timer.start_timer(id);

        std::thread::sleep(Duration::from_secs(3));

        assert_eq!(timer.check_timer(id, Duration::from_secs(5)), false);
    }
}
