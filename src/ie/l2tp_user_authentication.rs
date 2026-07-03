//! L2TP User Authentication IE (Extendable, IE Type 278).
//!
//! Per 3GPP TS 29.244 Clause 8.2.187, contains L2TP PPP authentication
//! parameters (Proxy Authen attributes per IETF RFC 2661).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L2tpUserAuthentication {
    /// Proxy Authen Type Value (2 bytes, e.g. 1=Text, 2=CHAP, 3=PAP, 5=MS-CHAPv1).
    pub proxy_authen_type: u16,
    /// Proxy Authen Name (PAN flag; length-prefixed bytes from RFC 2661).
    pub proxy_authen_name: Option<Vec<u8>>,
    /// Proxy Authen Challenge (PAC flag; length-prefixed bytes from RFC 2661).
    pub proxy_authen_challenge: Option<Vec<u8>>,
    /// Proxy Authen Response (PAR flag; length-prefixed bytes from RFC 2661).
    pub proxy_authen_response: Option<Vec<u8>>,
    /// Proxy Authen ID (PAI flag; 2-byte session identifier from RFC 2661).
    pub proxy_authen_id: Option<u16>,
}

impl L2tpUserAuthentication {
    pub fn new(proxy_authen_type: u16) -> Self {
        Self {
            proxy_authen_type,
            proxy_authen_name: None,
            proxy_authen_challenge: None,
            proxy_authen_response: None,
            proxy_authen_id: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        // flags byte: PAN=bit1, PAC=bit2, PAR=bit3, PAI=bit4
        let mut flags: u8 = 0;
        if self.proxy_authen_name.is_some() {
            flags |= 0x01;
        }
        if self.proxy_authen_challenge.is_some() {
            flags |= 0x02;
        }
        if self.proxy_authen_response.is_some() {
            flags |= 0x04;
        }
        if self.proxy_authen_id.is_some() {
            flags |= 0x08;
        }

        let mut buf = Vec::new();
        buf.extend_from_slice(&self.proxy_authen_type.to_be_bytes());
        buf.push(flags);

        if let Some(ref name) = self.proxy_authen_name {
            buf.extend_from_slice(&(name.len() as u16).to_be_bytes());
            buf.extend_from_slice(name);
        }
        if let Some(ref challenge) = self.proxy_authen_challenge {
            buf.extend_from_slice(&(challenge.len() as u16).to_be_bytes());
            buf.extend_from_slice(challenge);
        }
        if let Some(ref response) = self.proxy_authen_response {
            buf.extend_from_slice(&(response.len() as u16).to_be_bytes());
            buf.extend_from_slice(response);
        }
        if let Some(id) = self.proxy_authen_id {
            buf.extend_from_slice(&id.to_be_bytes());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "L2tpUserAuthentication",
                IeType::L2tpUserAuthentication,
                3,
                data.len(),
            ));
        }
        let proxy_authen_type = u16::from_be_bytes([data[0], data[1]]);
        let flags = data[2];
        let mut pos = 3;
        let mut result = Self::new(proxy_authen_type);

        macro_rules! read_len_prefixed {
            ($field:expr) => {{
                if data.len() < pos + 2 {
                    return Err(PfcpError::invalid_length(
                        "L2tpUserAuthentication",
                        IeType::L2tpUserAuthentication,
                        pos + 2,
                        data.len(),
                    ));
                }
                let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
                pos += 2;
                if data.len() < pos + len {
                    return Err(PfcpError::invalid_length(
                        "L2tpUserAuthentication",
                        IeType::L2tpUserAuthentication,
                        pos + len,
                        data.len(),
                    ));
                }
                let bytes = data[pos..pos + len].to_vec();
                pos += len;
                bytes
            }};
        }

        if flags & 0x01 != 0 {
            result.proxy_authen_name = Some(read_len_prefixed!("name"));
        }
        if flags & 0x02 != 0 {
            result.proxy_authen_challenge = Some(read_len_prefixed!("challenge"));
        }
        if flags & 0x04 != 0 {
            result.proxy_authen_response = Some(read_len_prefixed!("response"));
        }
        if flags & 0x08 != 0 {
            if data.len() < pos + 2 {
                return Err(PfcpError::invalid_length(
                    "L2tpUserAuthentication",
                    IeType::L2tpUserAuthentication,
                    pos + 2,
                    data.len(),
                ));
            }
            result.proxy_authen_id = Some(u16::from_be_bytes([data[pos], data[pos + 1]]));
        }
        Ok(result)
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::L2tpUserAuthentication, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_type_only() {
        let original = L2tpUserAuthentication::new(1);
        let parsed = L2tpUserAuthentication::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_with_name_and_response() {
        let mut original = L2tpUserAuthentication::new(1);
        original.proxy_authen_name = Some(b"user@example.com".to_vec());
        original.proxy_authen_response = Some(vec![0xAB, 0xCD, 0xEF]);
        let parsed = L2tpUserAuthentication::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_chap_full() {
        let mut original = L2tpUserAuthentication::new(2);
        original.proxy_authen_name = Some(b"alice".to_vec());
        original.proxy_authen_challenge = Some(vec![0x01, 0x02, 0x03, 0x04]);
        original.proxy_authen_response = Some(vec![0x10, 0x20, 0x30]);
        original.proxy_authen_id = Some(0x0042);
        let parsed = L2tpUserAuthentication::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer_rejected() {
        let result = L2tpUserAuthentication::unmarshal(&[0x00, 0x01]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = L2tpUserAuthentication::new(3).to_ie();
        assert_eq!(ie.ie_type, IeType::L2tpUserAuthentication);
    }
}
