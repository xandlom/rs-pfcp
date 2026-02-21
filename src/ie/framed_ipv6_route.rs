//! Framed-IPv6-Route Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.122, contains a Framed-IPv6-Route AVP value as a string.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedIpv6Route {
    pub value: String,
}

impl FramedIpv6Route {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let value = String::from_utf8(data.to_vec()).map_err(|e| {
            PfcpError::encoding_error("Framed-IPv6-Route", IeType::FramedIpv6Route, e.utf8_error())
        })?;
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FramedIpv6Route, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let route = FramedIpv6Route::new("2001:db8::/32 2001:db8::1");
        let parsed = FramedIpv6Route::unmarshal(&route.marshal()).unwrap();
        assert_eq!(parsed, route);
    }

    #[test]
    fn test_unmarshal_invalid_utf8() {
        assert!(matches!(
            FramedIpv6Route::unmarshal(&[0xFF, 0xFE]),
            Err(PfcpError::EncodingError { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            FramedIpv6Route::new("test").to_ie().ie_type,
            IeType::FramedIpv6Route
        );
    }
}
