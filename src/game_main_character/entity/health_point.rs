#[derive(Debug, Clone)]
pub struct HealthPoint {
    number: i32,
}

impl HealthPoint {
    pub fn new(initial_health: i32) -> HealthPoint {
        HealthPoint { number: initial_health }
    }

    pub fn get_health(&self) -> i32 {
        self.number
    }

    pub fn lose_health(&mut self, damage: i32) { self.number -= damage }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_point() {
        let mut health_point = HealthPoint::new(100);
        println!("HealthPoint: {}", health_point.get_health());
        assert_eq!(health_point.get_health(), 100);

        health_point.lose_health(5);
        assert_eq!(health_point.get_health(), 95)
    }
}
