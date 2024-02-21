#[derive(Clone, Debug, PartialEq)]
pub enum ExtraEffect {
    Dummy,
    Freeze = 1,
    DarkFire = 2,
}

impl From<i32> for ExtraEffect {
    fn from(value: i32) -> Self {
        match value {
            1 => ExtraEffect::Freeze,
            2 => ExtraEffect::DarkFire,
            _ => ExtraEffect::Dummy,
        }
    }
}
