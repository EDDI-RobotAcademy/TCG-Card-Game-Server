#[derive(PartialEq, Debug, Clone)]
pub enum ActiveSkillType {
    Dummy = 0,
    SingleTarget = 1,
    BroadArea = 2,
    DoubleTarget = 3,
}