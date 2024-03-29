#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FalseMessage {
    Dummy = -1,

    DeployedRoundAttack = 0,
    NotEnoughSkillEnergy = 1,
    UnitActionLimitOver = 2,
    NotYourTurn = 3,
    SupportUsageOver = 4,

    NotEnoughBasicAttackEnergy = 11,
    MythicalCardRoundLimit = 12,
    NotYourTurnFieldEnergy = 13,
    UnattackableUnit = 14,
    UnitFrozen = 15,


}

impl From<i32> for FalseMessage {
    fn from(value: i32) -> Self {
        match value {
            -1 => FalseMessage::Dummy,
            0 => FalseMessage::DeployedRoundAttack,
            1 => FalseMessage::NotEnoughSkillEnergy,
            2 => FalseMessage::UnitActionLimitOver,
            3 => FalseMessage::NotYourTurn,
            4 => FalseMessage::SupportUsageOver,
            11 => FalseMessage::NotEnoughBasicAttackEnergy,
            12 => FalseMessage::MythicalCardRoundLimit,
            13 => FalseMessage::NotYourTurnFieldEnergy,
            14 => FalseMessage::UnattackableUnit,
            15 => FalseMessage::UnitFrozen,

            _ => panic!("Invalid enum value"),
        }
    }
}