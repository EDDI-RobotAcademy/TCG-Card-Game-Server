use serde_json::Value as JsonValue;
use crate::battle_room::service::request::what_is_the_room_number_request::WhatIsTheRoomNumberRequest;

pub fn create_what_is_the_room_number_request(data: &JsonValue) -> Option<WhatIsTheRoomNumberRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(WhatIsTheRoomNumberRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

