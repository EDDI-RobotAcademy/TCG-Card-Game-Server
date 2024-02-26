use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusMainCharacterEnum {
    Dummy = 0,
    Survival = 1,
    Death = 2,
}