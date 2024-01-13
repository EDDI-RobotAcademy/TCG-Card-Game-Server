use std::net::UdpSocket;

pub struct IPAddress;

impl IPAddress {
    pub fn get_local_ip_from_google() -> Option<String> {
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if let Ok(_) = socket.connect("8.8.8.8:80") {
                if let Ok(local_addr) = socket.local_addr() {
                    return Some(local_addr.ip().to_string());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        if let Some(ip) = IPAddress::get_local_ip_from_google() {
            println!("Local IP: {}", ip);
            assert!(!ip.is_empty());
        } else {
            panic!("Failed to get local IP");
        }
    }
}