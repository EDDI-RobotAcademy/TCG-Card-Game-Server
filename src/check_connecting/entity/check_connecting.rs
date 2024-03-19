use crate::check_connecting::entity::check_connecting_status::ConnectStatus;

#[derive(Debug, PartialEq)]
pub struct ConnectMonitor {
    pub address: String,
    pub connect_status: ConnectStatus
}

impl ConnectMonitor {
    pub fn new(address: &str, connect_status: ConnectStatus) -> Self {
        ConnectMonitor {
            address: address.to_string(),
            connect_status,
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_connect_status(&self) -> &ConnectStatus { &self.connect_status }

    pub fn set_connect_status(&mut self, connect_monitor_status: ConnectStatus) {
        self.connect_status = connect_monitor_status;
    }
}