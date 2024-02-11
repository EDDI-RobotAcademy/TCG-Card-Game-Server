use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::effect::Effect;
use crate::game_card_energy::entity::energy_card::EnergyCard;
use crate::game_card_energy::entity::status_effect::StatusEffect;

pub struct SummaryEnergyCardEffect {
    energy_card: EnergyCard,
    // 상태 이상
    status_effects: Vec<StatusEffect>,
}

impl SummaryEnergyCardEffect {
    pub fn new(race: RaceEnum, effects: Vec<StatusEffect>) -> Self {
        SummaryEnergyCardEffect {
            energy_card: EnergyCard::new(race),
            status_effects: effects,
        }
    }

    pub fn get_energy_card(&self) -> &EnergyCard {
        &self.energy_card
    }

    pub fn get_status_effects(&self) -> &Vec<StatusEffect> {
        &self.status_effects
    }
}

#[cfg(test)]
mod tests {
    use crate::game_card_energy::entity::effect::Effect::Darkfire;
    use super::*;

    #[test]
    fn test_summary_energy_card_effect() {
        let race = RaceEnum::Undead;
        let status_effects = vec![
            StatusEffect::new(Darkfire, 3, 10, -1),
        ];
        let summary_energy_card_effect = SummaryEnergyCardEffect::new(race, status_effects.clone());

        assert_eq!(summary_energy_card_effect.get_energy_card().get_race(), &race);
        assert_eq!(summary_energy_card_effect.get_status_effects(), &status_effects);

        println!("Energy Card Race: {:?}", summary_energy_card_effect.get_energy_card().get_race());
        println!("Status Effects: {:?}", summary_energy_card_effect.get_status_effects());
    }
}
