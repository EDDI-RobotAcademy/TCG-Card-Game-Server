use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

#[async_trait]
pub trait NotifyPlayerActionInfoRepository {
    async fn notify_player_draw_card_with_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        drawn_card_list: Vec<i32>) -> bool;
}