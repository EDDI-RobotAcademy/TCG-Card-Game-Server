#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldEnergyAdditionCalculator {
    divider: i32,
}

impl FieldEnergyAdditionCalculator {
    pub fn new(divider: i32) -> Self {
        FieldEnergyAdditionCalculator {
            divider,
        }
    }
    pub fn init_calculator_by_info(&mut self, divider: i32) {
        self.divider = divider
    }
    pub fn calculation_of_field_energy_increment(&self, current_health_point_of_unit: i32) -> i32 {
        current_health_point_of_unit / self.divider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_of_field_energy_increment_with_current_health_point_of_field_unit() {
        let mut calculator = FieldEnergyAdditionCalculator::new(5);

        let result = calculator.calculation_of_field_energy_increment(13);

        assert_eq!(result, 2)
    }
}