#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RaceEnum {
    Dummy = 0,
    Undead = 1,
    Human = 2,
    Trent = 3,
    Angel = 4,
    Machine = 5,
    Chaos = 6,
}

impl From<i32> for RaceEnum {
    fn from(value: i32) -> Self {
        match value {
            1 => RaceEnum::Undead,
            2 => RaceEnum::Human,
            3 => RaceEnum::Trent,
            4 => RaceEnum::Angel,
            5 => RaceEnum::Machine,
            6 => RaceEnum::Chaos,
            _ => panic!("Invalid enum value"),
        }
    }
}