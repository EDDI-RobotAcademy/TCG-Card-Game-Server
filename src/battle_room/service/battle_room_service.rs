use async_trait::async_trait;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::battle_room::service::request::what_is_the_room_number_request::WhatIsTheRoomNumberRequest;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::battle_room::service::response::find_opponent_by_account_id_response::FindOpponentByAccountIdResponse;
use crate::battle_room::service::response::what_is_the_room_number_response::WhatIsTheRoomNumberResponse;

#[async_trait]
pub trait BattleRoomService {
    // async fn enqueue_player_id_to_wait_queue(&self, battle_match_request: BattleMatchRequest) -> BattleMatchResponse;
    async fn what_is_the_room_number(&self, what_is_the_room_number_request: WhatIsTheRoomNumberRequest) -> WhatIsTheRoomNumberResponse;
    async fn find_opponent_by_account_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> FindOpponentByAccountIdResponse;
}