use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;

pub struct SummaryItemCardEffectResponse {
    // TODO: 향후 VO 배치 필요
    required_energy_race: RaceEnum,
    required_energy_count: i32,
    alternatives_damage: i32,
    apply_target_minimum_grade: GradeEnum
}

impl SummaryItemCardEffectResponse {
    pub fn new(
        required_energy_race: RaceEnum,
        required_energy_count: i32,
        alternatives_damage: i32,
        apply_target_minimum_grade: GradeEnum) -> Self {

            Self {
                required_energy_race,
                required_energy_count,
                alternatives_damage,
                apply_target_minimum_grade }
    }

    pub fn required_energy_race(&self) -> RaceEnum {
        self.required_energy_race
    }

    pub fn required_energy_count(&self) -> i32 {
        self.required_energy_count
    }

    pub fn alternatives_damage(&self) -> i32 {
        self.alternatives_damage
    }

    pub fn apply_target_minimum_grade(&self) -> GradeEnum {
        self.apply_target_minimum_grade
    }

    pub fn from_summary_item_card_effect(game_card_item_effect: GameCardItemEffect) -> SummaryItemCardEffectResponse {
        let required_energy = game_card_item_effect.get_required_energy();
        SummaryItemCardEffectResponse::new(
            *required_energy.get_required_energy_race(),
            required_energy.get_required_energy_count(),
            game_card_item_effect.get_alternatives_damage(),
            game_card_item_effect.get_apply_target_minimum_grade())
    }
}