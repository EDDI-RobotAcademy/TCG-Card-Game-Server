#[derive(Debug, Clone, PartialEq)]
pub enum PassiveSkillCastingCondition {
    Dummy = 0,
    Deploy = 1,
    TurnStart = 2,
    TurnEnd = 3,
}