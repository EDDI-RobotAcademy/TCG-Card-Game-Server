use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Serialize, Deserialize)]
pub enum CardDictionaryLabel {
    Name,
    Race,
    Grade,
    Kind,
    ActivationEnergy,
    AttackPoint,
    HealthPoint,
}