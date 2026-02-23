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

impl TryFrom<u8> for RedirectAddressType {
    type Error = PfcpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RedirectAddressType::Ipv4),
            1 => Ok(RedirectAddressType::Ipv6),
            2 => Ok(RedirectAddressType::Url),
            3 => Ok(RedirectAddressType::SipUri),
            _ => Err(PfcpError::invalid_value(
                "RedirectAddressType",
                value.to_string(),
                "unknown redirect address type; valid values are 0=IPv4, 1=IPv6, 2=URL, 3=SIP URI",
            )),
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
        let address_type = RedirectAddressType::try_from(data[0])?;
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

    #[test]
    fn test_redirect_information_unknown_address_type() {
        // Unknown address type must return error, not panic
        let data = vec![0xFF, b'1', b'.', b'2', b'.', b'3', b'.', b'4'];
        let result = RedirectInformation::unmarshal(&data);
        assert!(matches!(result, Err(PfcpError::InvalidValue { .. })));
    }
}
