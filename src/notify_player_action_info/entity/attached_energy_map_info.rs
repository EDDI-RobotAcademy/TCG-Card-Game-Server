use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachedEnergyMapInfo {
    attached_energy_map: HashMap<i32, i32>
}

impl AttachedEnergyMapInfo {
    pub fn new(attached_energy_map: HashMap<i32, i32>) -> Self {
        AttachedEnergyMapInfo {
            attached_energy_map
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
        let object = AttachedEnergyMapInfo::new(attached_energy_map);
        println!("{:?}", json!(object).get("attached_energy_map").unwrap());
    }
}