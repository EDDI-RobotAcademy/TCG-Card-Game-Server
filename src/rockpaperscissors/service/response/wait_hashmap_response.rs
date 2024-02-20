#[derive(Debug)]
pub struct WaitHashmapResponse {
    is_success:bool,

}

impl WaitHashmapResponse {
    pub fn new(is_success:bool) -> Self {
        WaitHashmapResponse {
            is_success
        }
    }

    pub fn get_is_success(&self) -> bool {
        self.is_success
    }

}
