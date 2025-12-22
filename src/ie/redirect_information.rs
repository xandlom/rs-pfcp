// src/ie/redirect_information.rs

//! Redirect Information Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectAddressType {
    Ipv4,
    Ipv6,
    Url,
    SipUri,
}

impl From<u8> for RedirectAddressType {
    fn from(value: u8) -> Self {
        match value {
            0 => RedirectAddressType::Ipv4,
            1 => RedirectAddressType::Ipv6,
            2 => RedirectAddressType::Url,
            3 => RedirectAddressType::SipUri,
            _ => panic!("Invalid RedirectAddressType"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedirectInformation {
    pub address_type: RedirectAddressType,
    pub server_address: String,
}

impl RedirectInformation {
    pub fn new(address_type: RedirectAddressType, server_address: &str) -> Self {
        RedirectInformation {
            address_type,
            server_address: server_address.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.address_type as u8);
        data.extend_from_slice(self.server_address.as_bytes());
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Redirect Information",
                IeType::RedirectInformation,
                2,
                data.len(),
            ));
        }
        let address_type = RedirectAddressType::from(data[0]);
        let server_address = String::from_utf8(data[1..].to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "Redirect Information",
                IeType::RedirectInformation,
                e.utf8_error(),
            )
        })?;
        Ok(RedirectInformation {
            address_type,
            server_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redirect_information_marshal_unmarshal_ipv4() {
        let ri = RedirectInformation::new(RedirectAddressType::Ipv4, "1.2.3.4");
        let marshaled = ri.marshal();
        let unmarshaled = RedirectInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ri);
    }

    #[test]
    fn test_redirect_information_marshal_unmarshal_url() {
        let ri = RedirectInformation::new(RedirectAddressType::Url, "http://example.com");
        let marshaled = ri.marshal();
        let unmarshaled = RedirectInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ri);
    }
}
