#[derive(Debug, Clone)]
// TODO: Passive Skill 과 Passive Status 가 구분되어야 Domain 표현이 좋았을 것
pub enum PassiveSkillType {
    Dummy = 0,
    BroadArea = 1,
    SingleTarget = 2,
    PhysicalImmunity = 3,
}