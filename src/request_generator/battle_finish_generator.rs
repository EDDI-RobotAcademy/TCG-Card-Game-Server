use serde_json::Value as JsonValue;
use crate::battle_finish::service::request::battle_finish_request::BattleFinishRequest;
use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum;

pub fn create_battle_finish_request(data: &JsonValue) -> Option<BattleFinishRequest> {
    if let (Some(sessionInfo), Some(finishPositionEnum)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("finishPositionEnum").and_then(|v|v.as_str())
    ) {
        let finish_position_enum_match = match finishPositionEnum {
            "Dummy" => Ok(FinishPositionEnum::Dummy),
            "Winner" => Ok(FinishPositionEnum::Winner),
            "Loser" => Ok(FinishPositionEnum::Loser),
            "Draw" => Ok(FinishPositionEnum::Draw),
            _ => Err(()),
        }.unwrap();

        Some(BattleFinishRequest::new(sessionInfo.to_string(), finish_position_enum_match))
    } else {
        None
    }
}
