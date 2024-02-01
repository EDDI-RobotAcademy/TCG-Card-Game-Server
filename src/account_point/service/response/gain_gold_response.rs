#[derive(Debug, Clone)]
pub struct GainGoldResponse {
    is_success: bool,
}

impl GainGoldResponse {
    pub fn new(is_success: bool) -> Self {
        GainGoldResponse { is_success }
    }
}