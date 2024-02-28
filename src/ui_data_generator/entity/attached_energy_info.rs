use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachedEnergyInfo {
    attached_energy_map: HashMap<i32, i32>,
    total_energy_count: i32,
}

impl AttachedEnergyInfo {
    pub fn new(attached_energy_map: HashMap<i32, i32>) -> Self {
        let mut total_energy_count = 0;
        for (_, energy_count) in &attached_energy_map {
            total_energy_count += energy_count;
        }
        AttachedEnergyInfo {
            attached_energy_map,
            total_energy_count
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        let mut attached_energy_map = HashMap::new();
        attached_energy_map.insert(1, 2);
        attached_energy_map.insert(2, 2);
        let object = AttachedEnergyInfo::new(attached_energy_map);
        println!("{:?}", json!(object).get("attached_energy_map").unwrap());
        println!("{:?}", json!(object).get("total_energy_count").unwrap());
    }
}