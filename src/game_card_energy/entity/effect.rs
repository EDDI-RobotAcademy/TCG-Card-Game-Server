#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    Dummy,
    Freeze = 1,
    Darkfire = 2,
}

impl Effect {
    pub fn to_i32(&self) -> i32 {
        match self {
            Effect::Dummy => 0,
            Effect::Freeze => 1,
            Effect::Darkfire => 2,
        }
    }
}
