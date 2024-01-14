
pub struct ReceiveData {
    receive_content: [u8; 1024]
}

impl ReceiveData {
    pub fn new() -> Self {
        ReceiveData {
            receive_content: [0; 1024],
        }
    }

    pub fn get_receive_content(&self) -> &[u8] {
        &self.receive_content
    }

    pub fn receive_content_mut(&mut self) -> &mut [u8] {
        &mut self.receive_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_data() {
        const HELLO_RUST: &[u8] = b"Hello, Rust!";
        let mut receive_data = ReceiveData::new();

        let len = HELLO_RUST.len();
        receive_data.receive_content_mut()[..len].copy_from_slice(HELLO_RUST);

        let stored_data = receive_data.get_receive_content();
        assert_eq!(&stored_data[..len], HELLO_RUST);
    }
}
