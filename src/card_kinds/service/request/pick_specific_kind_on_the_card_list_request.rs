use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

#[derive(Debug)]
pub struct PickSpecificKindOnTheCardListRequest {
    card_list: Vec<i32>,
    specific_kind: KindsEnum,
}

impl PickSpecificKindOnTheCardListRequest {
    pub fn new(card_list: Vec<i32>, specific_kind: KindsEnum) -> Self {
        PickSpecificKindOnTheCardListRequest {
            card_list,
            specific_kind,
        }
    }
    pub fn get_card_list(&self) -> &Vec<i32> { &self.card_list }
    pub fn get_specific_kind(&self) -> &KindsEnum { &self.specific_kind }
}