#[derive(Debug)]
pub struct WhatIsTheRoomNumberRequest {
    session_id: String,
}

impl WhatIsTheRoomNumberRequest {
    pub fn new(session_id: String) -> Self {
        WhatIsTheRoomNumberRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}