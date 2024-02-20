use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsUnitBasicAttackPossibleResponse {
    is_possible: bool,
}

impl IsUnitBasicAttackPossibleResponse {
    pub fn new(is_possible: bool) -> Self { IsUnitBasicAttackPossibleResponse { is_possible } }
    pub fn is_possible(&self) -> bool { self.is_possible }
}