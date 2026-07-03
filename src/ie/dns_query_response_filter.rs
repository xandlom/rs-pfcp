//! DNS Query/Response Filter IE (Extendable, IE Type 294).
//!
//! Per 3GPP TS 29.244 Clause 8.2.201, contains a DNS Query or Response Filter.
//! The Domain Name Pattern field is a JSON object (FqdnPatternMatchingRule)
//! per 3GPP TS 29.571 Clause 5.2.4.23.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQueryResponseFilter {
    /// Flags byte (octet 5): filter type and direction flags.
    pub flags: u8,
    /// Domain Name Pattern as raw bytes (JSON FqdnPatternMatchingRule object).
    /// Located at octets 7+ (after 1 flags byte + 1 spare byte).
    pub domain_name_pattern: Vec<u8>,
}

impl DnsQueryResponseFilter {
    pub fn new(flags: u8, domain_name_pattern: impl Into<Vec<u8>>) -> Self {
        Self {
            flags,
            domain_name_pattern: domain_name_pattern.into(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2 + self.domain_name_pattern.len());
        buf.push(self.flags);
        buf.push(0x00); // spare octet (octet 6)
        buf.extend_from_slice(&self.domain_name_pattern);
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "DnsQueryResponseFilter",
                IeType::DnsQueryResponseFilter,
                2,
                data.len(),
            ));
        }
        let flags = data[0];
        // data[1] is spare, ignored on receipt
        let domain_name_pattern = data[2..].to_vec();
        Ok(Self {
            flags,
            domain_name_pattern,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DnsQueryResponseFilter, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_with_pattern() {
        let pattern = br#"{"fqdn":"*.example.com"}"#.to_vec();
        let original = DnsQueryResponseFilter::new(0x01, pattern);
        let parsed = DnsQueryResponseFilter::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_flags_only() {
        let original = DnsQueryResponseFilter::new(0x02, vec![]);
        let parsed = DnsQueryResponseFilter::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer_rejected() {
        let result = DnsQueryResponseFilter::unmarshal(&[0x01]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_empty_rejected() {
        let result = DnsQueryResponseFilter::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = DnsQueryResponseFilter::new(0x00, vec![]).to_ie();
        assert_eq!(ie.ie_type, IeType::DnsQueryResponseFilter);
    }
}
