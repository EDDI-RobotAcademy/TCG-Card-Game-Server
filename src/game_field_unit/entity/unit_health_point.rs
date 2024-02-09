#[derive(Debug, Clone)]
pub struct UnitHealthPoint {
    max_health_point: i32,
    current_health_point: i32,
}

impl UnitHealthPoint {
    pub fn new(max_health_point: i32) -> Self {
        Self {
            max_health_point,
            current_health_point: max_health_point
        }
    }

    pub fn max_health_point(&self) -> i32 {
        self.max_health_point
    }

    pub fn current_health_point(&self) -> i32 {
        self.current_health_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_health_point_creation() {
        let max_health_point = 100;
        let unit_health_point = UnitHealthPoint::new(max_health_point);

        assert_eq!(unit_health_point.max_health_point(), max_health_point);
        assert_eq!(unit_health_point.current_health_point(), max_health_point);

        println!("unit_health_point: {:?}", unit_health_point);
    }
}

