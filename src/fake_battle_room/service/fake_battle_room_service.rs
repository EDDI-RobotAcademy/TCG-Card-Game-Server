use async_trait::async_trait;
use crate::fake_battle_room::service::request::create_battle_fake_room_request::CreateFakeBattleRoomRequest;
use crate::fake_battle_room::service::response::create_battle_fake_room_response::CreateFakeBattleRoomResponse;

#[async_trait]
pub trait FakeBattleRoomService {
    // TODO: 추후 1 대 1 배틀, 다 대 다, 레이드 등등을 고려할 필요가 있음 (네이밍)
    async fn create_fake_battle_room(
        &self,
        create_fake_battle_room_request: CreateFakeBattleRoomRequest)
        -> CreateFakeBattleRoomResponse;
}