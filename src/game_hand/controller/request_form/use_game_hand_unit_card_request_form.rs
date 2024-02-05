use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

#[derive(Debug)]
pub struct UseGameHandUnitCardRequestForm {
    session_id: String,
    unit_number: String,
}

impl UseGameHandUnitCardRequestForm {
    pub fn new(session_id: String, unit_number: String) -> Self {
        UseGameHandUnitCardRequestForm {
            session_id,
            unit_number,
        }
    }
    pub fn to_use_game_hand_unit_card_request(&self) -> UseGameHandUnitCardRequest {
        UseGameHandUnitCardRequest::new( self.session_id.clone(), self.unit_number.clone() )
    }
}