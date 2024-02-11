use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::effect::Effect;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_energy::handler::game_card_energy_handler::GameCardEnergyHandler;

pub struct EnergyCard_151_Function;

impl GameCardEnergyHandler for EnergyCard_151_Function {
    unsafe fn use_specific_energy_card(&self) -> SummaryEnergyCardEffect {
        println!("SupportCard_151_Function: use_specific_support_card()");

        SummaryEnergyCardEffect::new(RaceEnum::Undead,
                                     vec![
                                         StatusEffect::new(Effect::Freeze, 1, -1, 2),
                                         StatusEffect::new(Effect::Darkfire, 3, 10, -1)
                                     ])
    }
}