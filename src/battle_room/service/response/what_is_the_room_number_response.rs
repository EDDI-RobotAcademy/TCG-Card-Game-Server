use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIsTheRoomNumberResponse {
    room_number: i32,
}

impl WhatIsTheRoomNumberResponse {
    pub fn new(room_number: i32) -> Self {
        WhatIsTheRoomNumberResponse { room_number }
    }
}
