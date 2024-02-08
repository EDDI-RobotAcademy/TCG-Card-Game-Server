use async_trait::async_trait;
use crate::game_card_energy::service::request::summary_energy_card_effect_request::SummaryEnergyCardEffectRequest;
use crate::game_card_energy::service::response::summary_energy_card_effect_response::SummaryEnergyCardEffectResponse;

#[async_trait]
pub trait GameCardEnergyService {
    async fn summary_energy_effect(&mut self, summary_energy_card_effect_request: SummaryEnergyCardEffectRequest) -> SummaryEnergyCardEffectResponse;
}
