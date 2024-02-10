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

    pub fn get_max_health_point(&self) -> i32 {
        self.max_health_point
    }

    pub fn get_current_health_point(&self) -> i32 {
        self.current_health_point
    }

    pub fn set_current_health_point(&mut self, current_health_point: i32) {
        self.current_health_point = current_health_point;
    }

    pub fn increase_max_health(&mut self, increase_point: i32) {
        self.max_health_point += increase_point;
        self.current_health_point += increase_point;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_health_point_creation() {
        let max_health_point = 100;
        let unit_health_point = UnitHealthPoint::new(max_health_point);

        assert_eq!(unit_health_point.get_max_health_point(), max_health_point);
        assert_eq!(unit_health_point.get_current_health_point(), max_health_point);

        println!("unit_health_point: {:?}", unit_health_point);
    }

    #[test]
    fn test_increase_unit_health_point_creation() {
        let max_health_point = 100;
        let mut unit_health_point = UnitHealthPoint::new(max_health_point);

        println!("before increase -> unit_health_point: {:?}", unit_health_point);

        unit_health_point.increase_max_health(10);
        assert_eq!(unit_health_point.get_max_health_point(), max_health_point + 10);
        assert_eq!(unit_health_point.get_current_health_point(), max_health_point + 10);

        println!("after increase -> unit_health_point: {:?}", unit_health_point);
    }
}

