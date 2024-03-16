use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KindsEnum {
    Dummy = 0,
    Unit = 1,
    Item = 2,
    Trap = 3,
    Support = 4,
    Tool = 5,
    Energy = 6,
    Environment = 7,
    Token = 8,
}

impl From<i32> for KindsEnum {
    fn from(value: i32) -> Self {
        match value {
            1 => KindsEnum::Unit,
            2 => KindsEnum::Item,
            3 => KindsEnum::Trap,
            4 => KindsEnum::Support,
            5 => KindsEnum::Tool,
            6 => KindsEnum::Energy,
            7 => KindsEnum::Environment,
            8 => KindsEnum::Token,
            _ => panic!("Invalid enum value"),
        }
    }
}