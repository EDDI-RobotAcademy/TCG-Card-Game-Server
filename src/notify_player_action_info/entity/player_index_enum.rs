use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerIndex {
    Dummy = 0,
    You = 1,
    Opponent = 2,
}

impl From<i32> for PlayerIndex {
    fn from(value: i32) -> Self {
        match value {
            1 => PlayerIndex::You,
            2 => PlayerIndex::Opponent,
            _ => PlayerIndex::Dummy,
        }
    }
}