use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DictionaryIndex {
    Name = 0,
    Race,
    Grade,
    Kind,
    ActivationEnergy,
    AttackPoint,
    HealthPoint,
}