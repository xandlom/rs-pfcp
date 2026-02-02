//! FQ-CSID (Fully Qualified Control and Service Instance Identifier) Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a Fully Qualified Control and Service Instance Identifier.
///
/// The FQ-CSID IE indicates the FQ-CSID(s) of the control plane nodes.
/// It contains a Node ID and one or more CSIDs associated with that node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FqCsid {
    pub node_id_type: NodeIdType,
    pub node_id: NodeId,
    pub csids: Vec<u16>,
}

/// Node ID type within FQ-CSID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeIdType {
    Ipv4 = 0,
    Ipv6 = 1,
    Fqdn = 2,
}

/// Node ID within FQ-CSID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeId {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Fqdn(String),
}

impl FqCsid {
    /// Creates a new FQ-CSID with IPv4 node ID.
    pub fn new_ipv4(addr: Ipv4Addr, csids: Vec<u16>) -> Self {
        FqCsid {
            node_id_type: NodeIdType::Ipv4,
            node_id: NodeId::Ipv4(addr),
            csids,
        }
    }

    /// Creates a new FQ-CSID with IPv6 node ID.
    pub fn new_ipv6(addr: Ipv6Addr, csids: Vec<u16>) -> Self {
        FqCsid {
            node_id_type: NodeIdType::Ipv6,
            node_id: NodeId::Ipv6(addr),
            csids,
        }
    }

    /// Creates a new FQ-CSID with FQDN node ID.
    pub fn new_fqdn(fqdn: String, csids: Vec<u16>) -> Self {
        FqCsid {
            node_id_type: NodeIdType::Fqdn,
            node_id: NodeId::Fqdn(fqdn),
            csids,
        }
    }

    /// Encodes FQDN according to DNS message format (RFC 1035 clause 3.1) without trailing zero.
    fn encode_fqdn(fqdn: &str) -> Vec<u8> {
        let mut encoded = Vec::new();

        if fqdn.is_empty() {
            return encoded;
        }

        for label in fqdn.split('.') {
            if label.len() > 63 {
                // Label too long, truncate
                encoded.push(63);
                encoded.extend_from_slice(&label.as_bytes()[..63]);
            } else if !label.is_empty() {
                encoded.push(label.len() as u8);
                encoded.extend_from_slice(label.as_bytes());
            }
        }

        encoded
    }

    /// Decodes FQDN from DNS message format.
    fn decode_fqdn(data: &[u8]) -> Result<String, PfcpError> {
        let mut result = String::new();
        let mut offset = 0;

        while offset < data.len() {
            let label_len = data[offset] as usize;
            offset += 1;

            if label_len == 0 {
                break; // End of FQDN
            }

            if offset + label_len > data.len() {
                return Err(PfcpError::invalid_value(
                    "FQ-CSID FQDN",
                    label_len.to_string(),
                    "label length exceeds available data",
                ));
            }

            if !result.is_empty() {
                result.push('.');
            }

            let label_bytes = &data[offset..offset + label_len];
            let label = String::from_utf8(label_bytes.to_vec()).map_err(|_| {
                PfcpError::invalid_value("FQ-CSID FQDN", "non-UTF8", "invalid UTF-8 in FQDN label")
            })?;
            result.push_str(&label);
            offset += label_len;
        }

        Ok(result)
    }

    /// Marshals the FQ-CSID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Number of CSIDs (4 bits) + Node ID Type (4 bits)
        let num_csids = self.csids.len().min(15) as u8; // Max 15 CSIDs in 4 bits
        let first_byte = (num_csids << 4) | (self.node_id_type as u8);
        data.push(first_byte);

        // Node ID based on type
        match &self.node_id {
            NodeId::Ipv4(addr) => {
                data.extend_from_slice(&addr.octets());
            }
            NodeId::Ipv6(addr) => {
                data.extend_from_slice(&addr.octets());
            }
            NodeId::Fqdn(fqdn) => {
                // Encode FQDN according to DNS message format (RFC 1035)
                let encoded_fqdn = Self::encode_fqdn(fqdn);
                data.extend_from_slice(&encoded_fqdn);
            }
        }

        // CSIDs (each CSID is 2 bytes, big endian)
        for csid in &self.csids {
            data.extend_from_slice(&csid.to_be_bytes());
        }

        data
    }

    /// Unmarshals an FQ-CSID from a byte slice.
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length("FQ-CSID", IeType::FqCsid, 1, 0));
        }

        let first_byte = data[0];
        let num_csids = (first_byte >> 4) as usize;
        let node_id_type_val = first_byte & 0x0F;

        let node_id_type = match node_id_type_val {
            0 => NodeIdType::Ipv4,
            1 => NodeIdType::Ipv6,
            2 => NodeIdType::Fqdn,
            _ => {
                return Err(PfcpError::invalid_value(
                    "FQ-CSID Node ID type",
                    node_id_type_val.to_string(),
                    "must be 0 (IPv4), 1 (IPv6), or 2 (FQDN)",
                ))
            }
        };

        let mut offset = 1;
        let (node_id, node_id_len) = match node_id_type {
            NodeIdType::Ipv4 => {
                if data.len() < offset + 4 {
                    return Err(PfcpError::invalid_length(
                        "FQ-CSID IPv4 Node ID",
                        IeType::FqCsid,
                        offset + 4,
                        data.len(),
                    ));
                }
                let mut octets = [0u8; 4];
                octets.copy_from_slice(&data[offset..offset + 4]);
                (NodeId::Ipv4(Ipv4Addr::from(octets)), 4)
            }
            NodeIdType::Ipv6 => {
                if data.len() < offset + 16 {
                    return Err(PfcpError::invalid_length(
                        "FQ-CSID IPv6 Node ID",
                        IeType::FqCsid,
                        offset + 16,
                        data.len(),
                    ));
                }
                let mut octets = [0u8; 16];
                octets.copy_from_slice(&data[offset..offset + 16]);
                (NodeId::Ipv6(Ipv6Addr::from(octets)), 16)
            }
            NodeIdType::Fqdn => {
                // Calculate FQDN length by finding where CSIDs start
                let csids_start_offset = data.len() - (num_csids * 2);
                if csids_start_offset <= offset {
                    return Err(PfcpError::invalid_length(
                        "FQ-CSID FQDN",
                        IeType::FqCsid,
                        offset + 1,
                        csids_start_offset,
                    ));
                }
                let fqdn_len = csids_start_offset - offset;
                let fqdn_bytes = &data[offset..offset + fqdn_len];
                let fqdn = Self::decode_fqdn(fqdn_bytes)?;
                (NodeId::Fqdn(fqdn), fqdn_len)
            }
        };
        offset += node_id_len;

        // Parse CSIDs
        let mut csids = Vec::new();
        for _ in 0..num_csids {
            if data.len() < offset + 2 {
                return Err(PfcpError::invalid_length(
                    "FQ-CSID CSID",
                    IeType::FqCsid,
                    offset + 2,
                    data.len(),
                ));
            }
            let mut csid_bytes = [0u8; 2];
            csid_bytes.copy_from_slice(&data[offset..offset + 2]);
            csids.push(u16::from_be_bytes(csid_bytes));
            offset += 2;
        }

        Ok(FqCsid {
            node_id_type,
            node_id,
            csids,
        })
    }

    /// Converts to an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FqCsid, self.marshal())
    }
}

impl From<(Ipv4Addr, Vec<u16>)> for FqCsid {
    fn from((addr, csids): (Ipv4Addr, Vec<u16>)) -> Self {
        FqCsid::new_ipv4(addr, csids)
    }
}

impl From<(Ipv6Addr, Vec<u16>)> for FqCsid {
    fn from((addr, csids): (Ipv6Addr, Vec<u16>)) -> Self {
        FqCsid::new_ipv6(addr, csids)
    }
}

impl From<(String, Vec<u16>)> for FqCsid {
    fn from((fqdn, csids): (String, Vec<u16>)) -> Self {
        FqCsid::new_fqdn(fqdn, csids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fqdn_encoding() {
        // Test DNS format encoding
        let encoded = FqCsid::encode_fqdn("example.com");
        // Should be: 7 "example" 3 "com" (without trailing zero)
        assert_eq!(
            encoded,
            vec![7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm']
        );

        let decoded = FqCsid::decode_fqdn(&encoded).unwrap();
        assert_eq!(decoded, "example.com");
    }

    #[test]
    fn test_fqdn_encoding_subdomain() {
        let encoded = FqCsid::encode_fqdn("test.example.com");
        // Should be: 4 "test" 7 "example" 3 "com"
        assert_eq!(
            encoded,
            vec![
                4, b't', b'e', b's', b't', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c',
                b'o', b'm'
            ]
        );

        let decoded = FqCsid::decode_fqdn(&encoded).unwrap();
        assert_eq!(decoded, "test.example.com");
    }

    #[test]
    fn test_fq_csid_ipv4() {
        let addr = Ipv4Addr::new(192, 168, 1, 100);
        let csids = vec![1, 2, 3];
        let fq_csid = FqCsid::new_ipv4(addr, csids);

        assert_eq!(fq_csid.node_id_type, NodeIdType::Ipv4);
        assert_eq!(fq_csid.node_id, NodeId::Ipv4(addr));
        assert_eq!(fq_csid.csids, vec![1, 2, 3]);

        let marshaled = fq_csid.marshal();
        // First byte: 3 CSIDs (3 << 4) + IPv4 type (0) = 0x30
        assert_eq!(marshaled[0], 0x30);
        // IPv4 address (4 octets)
        assert_eq!(&marshaled[1..5], &[192, 168, 1, 100]);
        // CSIDs (big endian)
        assert_eq!(&marshaled[5..7], &[0, 1]);
        assert_eq!(&marshaled[7..9], &[0, 2]);
        assert_eq!(&marshaled[9..11], &[0, 3]);

        let unmarshaled = FqCsid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fq_csid);
    }

    #[test]
    fn test_fq_csid_ipv6() {
        let addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
        let csids = vec![0x1234, 0x5678];
        let fq_csid = FqCsid::new_ipv6(addr, csids);

        let marshaled = fq_csid.marshal();
        // First byte: 2 CSIDs (2 << 4) + IPv6 type (1) = 0x21
        assert_eq!(marshaled[0], 0x21);
        assert_eq!(marshaled.len(), 1 + 16 + 4); // header + IPv6 (16 octets) + 2 CSIDs

        let unmarshaled = FqCsid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fq_csid);
    }

    #[test]
    fn test_fq_csid_fqdn() {
        let fqdn = "example.com".to_string();
        let csids = vec![42];
        let fq_csid = FqCsid::new_fqdn(fqdn.clone(), csids);

        let marshaled = fq_csid.marshal();
        // First byte: 1 CSID (1 << 4) + FQDN type (2) = 0x12
        assert_eq!(marshaled[0], 0x12);
        // FQDN in DNS format: 7 "example" 3 "com"
        let expected_fqdn = vec![
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
        ];
        assert_eq!(&marshaled[1..1 + expected_fqdn.len()], &expected_fqdn);
        // CSID
        let csid_offset = 1 + expected_fqdn.len();
        assert_eq!(&marshaled[csid_offset..csid_offset + 2], &[0, 42]);

        let unmarshaled = FqCsid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fq_csid);
    }

    #[test]
    fn test_fq_csid_to_ie() {
        let addr = Ipv4Addr::new(10, 0, 0, 1);
        let fq_csid = FqCsid::new_ipv4(addr, vec![100]);
        let ie = fq_csid.to_ie();

        assert_eq!(ie.ie_type, IeType::FqCsid);
        assert_eq!(ie.payload, fq_csid.marshal());
    }

    #[test]
    fn test_fq_csid_unmarshal_errors() {
        // Empty data
        let result = FqCsid::unmarshal(&[]);
        assert!(result.is_err());

        // Invalid node ID type
        let result = FqCsid::unmarshal(&[0x03]); // Node ID type 3 is invalid
        assert!(result.is_err());

        // IPv4 but insufficient data
        let result = FqCsid::unmarshal(&[0x10, 192, 168]); // IPv4 type but only 2 bytes
        assert!(result.is_err());

        // IPv6 but insufficient data
        let result = FqCsid::unmarshal(&[0x11, 0x20, 0x01]); // IPv6 type but only 2 bytes
        assert!(result.is_err());

        // CSIDs but insufficient data
        let result = FqCsid::unmarshal(&[0x10, 192, 168, 1, 1, 0]); // 1 CSID but only 1 byte
        assert!(result.is_err());
    }

    #[test]
    fn test_fq_csid_round_trip() {
        let test_cases = vec![
            FqCsid::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), vec![1, 2, 3, 4]),
            FqCsid::new_ipv6(Ipv6Addr::LOCALHOST, vec![0xFFFF, 0x1234]),
            FqCsid::new_fqdn("test.example.com".to_string(), vec![42, 84, 126]),
            FqCsid::new_ipv4(Ipv4Addr::new(203, 0, 113, 1), vec![]), // No CSIDs
            FqCsid::new_fqdn("a.b.c.d".to_string(), vec![1]),        // Multi-level domain
        ];

        for fq_csid in test_cases {
            let marshaled = fq_csid.marshal();
            let unmarshaled = FqCsid::unmarshal(&marshaled).unwrap();
            assert_eq!(fq_csid, unmarshaled);
        }
    }

    #[test]
    fn test_fqdn_edge_cases() {
        // Empty FQDN
        let encoded = FqCsid::encode_fqdn("");
        assert_eq!(encoded, Vec::<u8>::new());
        let decoded = FqCsid::decode_fqdn(&[]).unwrap();
        assert_eq!(decoded, "");

        // Single label
        let encoded = FqCsid::encode_fqdn("localhost");
        assert_eq!(
            encoded,
            vec![9, b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't']
        );
        let decoded = FqCsid::decode_fqdn(&encoded).unwrap();
        assert_eq!(decoded, "localhost");
    }
}
