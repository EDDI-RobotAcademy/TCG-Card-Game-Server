use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_hand_card_use_info::PlayerHandCardUseInfo;

#[async_trait]
pub trait UiDataGeneratorRepository {
    async fn generate_use_energy_card_to_my_specific_unit_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerFieldUnitEnergyInfo, PlayerHandCardUseInfo, PlayerFieldUnitEnergyInfo);
}