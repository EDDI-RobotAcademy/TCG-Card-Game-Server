#[derive(PartialEq, Debug, Clone)]
pub enum ActiveSkillType {
    Dummy = 0,
    BroadArea = 1,
    SingleTarget = 2,
    DoubleTargetAttack = 3,
}