#[derive(Debug, Clone, PartialEq)]
pub enum PassiveSkillType {
    Dummy = 0,
    SingleTarget = 1,
    BroadArea = 2,
    DoubleTarget = 3,
}