#[derive(Debug, PartialEq)]
pub struct SupportCardUsageCounter {
    // 턴 당 사용 횟수는 바뀔 수 있으므로 integer 로 처리
    count: i32,
}

impl SupportCardUsageCounter {
    pub fn new() -> Self { SupportCardUsageCounter { count: 0 } }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn add_count(&mut self) {
        self.count += 1;
    }
}