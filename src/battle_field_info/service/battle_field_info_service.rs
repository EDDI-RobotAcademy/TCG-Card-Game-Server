use async_trait::async_trait;
use crate::battle_field_info::service::request::remain_deck_card_count_request::RemainDeckCardCountRequest;
use crate::battle_field_info::service::response::remain_deck_card_count_response::RemainDeckCardCountResponse;


#[async_trait]
pub trait BattleFieldInfoService {
    async fn get_remain_deck_card_count(&mut self, get_my_remain_deck_card_count_request: RemainDeckCardCountRequest) -> RemainDeckCardCountResponse;

}
