
pub struct TransmitData {
    transmit_content: [u8; 1024]
}

impl TransmitData {
    pub fn new() -> Self {
        TransmitData {
            transmit_content: [0; 1024],
        }
    }

    pub fn transmit_content(&self) -> [u8; 1024] {
        self.transmit_content
    }

    pub fn set_transmit_content(&mut self, transmit_content: [u8; 1024]) {
        self.transmit_content = transmit_content;
    }

    pub fn transmit_content_mut(&mut self) -> &mut [u8] {
        &mut self.transmit_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmit_data() {
        const HELLO_RUST: &[u8] = b"Hello, Rust!";
        let mut transmit_data = TransmitData::new();

        let len = HELLO_RUST.len();
        transmit_data.transmit_content_mut()[..len].copy_from_slice(HELLO_RUST);

        let sent_data = transmit_data.transmit_content();
        assert_eq!(&sent_data[..len], HELLO_RUST);
    }
}
