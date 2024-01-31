#[derive(Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_point() {
        let health_point = HealthPoint::new(100);
        println!("HealthPoint: {}", health_point.get_health());
        assert_eq!(health_point.get_health(), 100);

        let health_point = HealthPoint::new(75);
        println!("HealthPoint: {}", health_point.get_health());
        assert_eq!(health_point.get_health(), 75);
    }
}
