//! Trace Information IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Trace Information Element.
/// Used for network debugging and tracing support in 5G networks.
/// Defined in 3GPP TS 29.244 Section 8.2.102.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceInformation {
    pub mcc_mnc: [u8; 3],  // Mobile Country Code and Mobile Network Code (PLMN ID)
    pub trace_id: [u8; 3], // Trace ID (24 bits)
    pub triggering_events: Vec<u8>, // List of triggering events (variable length)
    pub trace_depth: u8,   // Trace depth
    pub list_of_interfaces: Vec<u8>, // List of interfaces to trace (variable length)
    pub ip_address_of_trace_collection_entity: Option<Vec<u8>>, // IPv4 or IPv6 address
}

impl TraceInformation {
    /// Creates a new Trace Information IE.
    pub fn new(
        mcc_mnc: [u8; 3],
        trace_id: [u8; 3],
        triggering_events: Vec<u8>,
        trace_depth: u8,
        list_of_interfaces: Vec<u8>,
    ) -> Self {
        TraceInformation {
            mcc_mnc,
            trace_id,
            triggering_events,
            trace_depth,
            list_of_interfaces,
            ip_address_of_trace_collection_entity: None,
        }
    }

    /// Adds an IP address of the trace collection entity.
    pub fn with_trace_collection_entity_ip(mut self, ip_address: Vec<u8>) -> Self {
        self.ip_address_of_trace_collection_entity = Some(ip_address);
        self
    }

    /// Sets the trace collection entity IPv4 address.
    pub fn with_trace_collection_entity_ipv4(mut self, ipv4: std::net::Ipv4Addr) -> Self {
        self.ip_address_of_trace_collection_entity = Some(ipv4.octets().to_vec());
        self
    }

    /// Sets the trace collection entity IPv6 address.
    pub fn with_trace_collection_entity_ipv6(mut self, ipv6: std::net::Ipv6Addr) -> Self {
        self.ip_address_of_trace_collection_entity = Some(ipv6.octets().to_vec());
        self
    }

    /// Gets the PLMN ID (MCC + MNC).
    pub fn plmn_id(&self) -> [u8; 3] {
        self.mcc_mnc
    }

    /// Gets the trace ID.
    pub fn trace_id(&self) -> [u8; 3] {
        self.trace_id
    }

    /// Gets the trace collection entity IP address as IPv4 if possible.
    pub fn trace_collection_entity_ipv4(&self) -> Option<std::net::Ipv4Addr> {
        if let Some(ref ip) = self.ip_address_of_trace_collection_entity {
            if ip.len() == 4 {
                return Some(std::net::Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]));
            }
        }
        None
    }

    /// Gets the trace collection entity IP address as IPv6 if possible.
    pub fn trace_collection_entity_ipv6(&self) -> Option<std::net::Ipv6Addr> {
        if let Some(ref ip) = self.ip_address_of_trace_collection_entity {
            if ip.len() == 16 {
                let mut octets = [0u8; 16];
                octets.copy_from_slice(ip);
                return Some(std::net::Ipv6Addr::from(octets));
            }
        }
        None
    }

    /// Gets the length of the marshaled Trace Information.
    pub fn len(&self) -> usize {
        let mut len = 3 + 3 + 1 + 1; // mcc_mnc + trace_id + triggering_events_len + trace_depth
        len += self.triggering_events.len();
        len += 1; // list_of_interfaces_len
        len += self.list_of_interfaces.len();

        if let Some(ref ip) = self.ip_address_of_trace_collection_entity {
            len += 1 + ip.len(); // ip_address_len + ip_address
        } else {
            len += 1; // ip_address_len = 0
        }

        len
    }

    /// Checks if the Trace Information is empty.
    pub fn is_empty(&self) -> bool {
        false // Trace Information always has mandatory fields
    }

    /// Marshals the Trace Information into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // PLMN ID (MCC + MNC)
        data.extend_from_slice(&self.mcc_mnc);

        // Trace ID
        data.extend_from_slice(&self.trace_id);

        // Triggering Events length and data
        data.push(self.triggering_events.len() as u8);
        data.extend_from_slice(&self.triggering_events);

        // Trace Depth
        data.push(self.trace_depth);

        // List of Interfaces length and data
        data.push(self.list_of_interfaces.len() as u8);
        data.extend_from_slice(&self.list_of_interfaces);

        // IP Address of Trace Collection Entity length and data
        if let Some(ref ip) = self.ip_address_of_trace_collection_entity {
            data.push(ip.len() as u8);
            data.extend_from_slice(ip);
        } else {
            data.push(0); // No IP address
        }

        data
    }

    /// Unmarshals a byte slice into a Trace Information IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Trace Information payload too short",
            ));
        }

        let mut offset = 0;

        // PLMN ID (MCC + MNC)
        let mut mcc_mnc = [0u8; 3];
        mcc_mnc.copy_from_slice(&payload[offset..offset + 3]);
        offset += 3;

        // Trace ID
        let mut trace_id = [0u8; 3];
        trace_id.copy_from_slice(&payload[offset..offset + 3]);
        offset += 3;

        // Triggering Events length
        let triggering_events_len = payload[offset] as usize;
        offset += 1;

        if offset + triggering_events_len > payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid triggering events length",
            ));
        }

        // Triggering Events data
        let triggering_events = payload[offset..offset + triggering_events_len].to_vec();
        offset += triggering_events_len;

        if offset >= payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing trace depth",
            ));
        }

        // Trace Depth
        let trace_depth = payload[offset];
        offset += 1;

        if offset >= payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing list of interfaces length",
            ));
        }

        // List of Interfaces length
        let interfaces_len = payload[offset] as usize;
        offset += 1;

        if offset + interfaces_len > payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid list of interfaces length",
            ));
        }

        // List of Interfaces data
        let list_of_interfaces = payload[offset..offset + interfaces_len].to_vec();
        offset += interfaces_len;

        if offset >= payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing IP address length",
            ));
        }

        // IP Address of Trace Collection Entity length
        let ip_len = payload[offset] as usize;
        offset += 1;

        let ip_address_of_trace_collection_entity = if ip_len > 0 {
            if offset + ip_len > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid IP address length",
                ));
            }
            Some(payload[offset..offset + ip_len].to_vec())
        } else {
            None
        };

        Ok(TraceInformation {
            mcc_mnc,
            trace_id,
            triggering_events,
            trace_depth,
            list_of_interfaces,
            ip_address_of_trace_collection_entity,
        })
    }

    /// Wraps the Trace Information in a Trace Information IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TraceInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_trace_information_marshal_unmarshal_minimal() {
        let mcc_mnc = [0x12, 0x34, 0x56];
        let trace_id = [0xAB, 0xCD, 0xEF];
        let triggering_events = vec![0x01, 0x02];
        let trace_depth = 5;
        let list_of_interfaces = vec![0x10, 0x20, 0x30];

        let trace_info = TraceInformation::new(
            mcc_mnc,
            trace_id,
            triggering_events.clone(),
            trace_depth,
            list_of_interfaces.clone(),
        );

        let marshaled = trace_info.marshal();
        let unmarshaled = TraceInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(trace_info, unmarshaled);
        assert_eq!(unmarshaled.mcc_mnc, mcc_mnc);
        assert_eq!(unmarshaled.trace_id, trace_id);
        assert_eq!(unmarshaled.triggering_events, triggering_events);
        assert_eq!(unmarshaled.trace_depth, trace_depth);
        assert_eq!(unmarshaled.list_of_interfaces, list_of_interfaces);
        assert_eq!(unmarshaled.ip_address_of_trace_collection_entity, None);
        assert!(!trace_info.is_empty());
    }

    #[test]
    fn test_trace_information_marshal_unmarshal_with_ipv4() {
        let mcc_mnc = [0x12, 0x34, 0x56];
        let trace_id = [0xAB, 0xCD, 0xEF];
        let triggering_events = vec![0x01];
        let trace_depth = 3;
        let list_of_interfaces = vec![0x10];
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);

        let trace_info = TraceInformation::new(
            mcc_mnc,
            trace_id,
            triggering_events.clone(),
            trace_depth,
            list_of_interfaces.clone(),
        )
        .with_trace_collection_entity_ipv4(ipv4);

        let marshaled = trace_info.marshal();
        let unmarshaled = TraceInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(trace_info, unmarshaled);
        assert_eq!(unmarshaled.trace_collection_entity_ipv4(), Some(ipv4));
        assert_eq!(unmarshaled.trace_collection_entity_ipv6(), None);
    }

    #[test]
    fn test_trace_information_marshal_unmarshal_with_ipv6() {
        let mcc_mnc = [0x12, 0x34, 0x56];
        let trace_id = [0xAB, 0xCD, 0xEF];
        let triggering_events = vec![0x01, 0x02, 0x03];
        let trace_depth = 7;
        let list_of_interfaces = vec![0x10, 0x20];
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0, 0, 0x8a2e, 0x370, 0x7334);

        let trace_info = TraceInformation::new(
            mcc_mnc,
            trace_id,
            triggering_events.clone(),
            trace_depth,
            list_of_interfaces.clone(),
        )
        .with_trace_collection_entity_ipv6(ipv6);

        let marshaled = trace_info.marshal();
        let unmarshaled = TraceInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(trace_info, unmarshaled);
        assert_eq!(unmarshaled.trace_collection_entity_ipv6(), Some(ipv6));
        assert_eq!(unmarshaled.trace_collection_entity_ipv4(), None);
    }

    #[test]
    fn test_trace_information_with_custom_ip() {
        let mcc_mnc = [0x01, 0x02, 0x03];
        let trace_id = [0x04, 0x05, 0x06];
        let triggering_events = vec![0xFF];
        let trace_depth = 1;
        let list_of_interfaces = vec![0xAA, 0xBB];
        let custom_ip = vec![0x10, 0x20, 0x30, 0x40, 0x50]; // 5-byte custom format

        let trace_info = TraceInformation::new(
            mcc_mnc,
            trace_id,
            triggering_events.clone(),
            trace_depth,
            list_of_interfaces.clone(),
        )
        .with_trace_collection_entity_ip(custom_ip.clone());

        let marshaled = trace_info.marshal();
        let unmarshaled = TraceInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(trace_info, unmarshaled);
        assert_eq!(
            unmarshaled.ip_address_of_trace_collection_entity,
            Some(custom_ip)
        );
        assert_eq!(unmarshaled.trace_collection_entity_ipv4(), None);
        assert_eq!(unmarshaled.trace_collection_entity_ipv6(), None);
    }

    #[test]
    fn test_trace_information_plmn_and_trace_id() {
        let mcc_mnc = [0xAA, 0xBB, 0xCC];
        let trace_id = [0x11, 0x22, 0x33];
        let trace_info = TraceInformation::new(mcc_mnc, trace_id, vec![], 0, vec![]);

        assert_eq!(trace_info.plmn_id(), mcc_mnc);
        assert_eq!(trace_info.trace_id(), trace_id);
    }

    #[test]
    fn test_trace_information_to_ie() {
        let trace_info = TraceInformation::new(
            [0x12, 0x34, 0x56],
            [0xAB, 0xCD, 0xEF],
            vec![0x01],
            3,
            vec![0x10],
        );

        let ie = trace_info.to_ie();
        assert_eq!(ie.ie_type, IeType::TraceInformation);

        let unmarshaled = TraceInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(trace_info, unmarshaled);
    }

    #[test]
    fn test_trace_information_unmarshal_too_short() {
        let result = TraceInformation::unmarshal(&[0x01, 0x02]); // Too short
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Trace Information payload too short"));
    }

    #[test]
    fn test_trace_information_unmarshal_invalid_triggering_events_len() {
        let mut payload = vec![0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF]; // mcc_mnc + trace_id
        payload.push(10); // triggering_events_len = 10, but not enough data
        payload.extend(vec![0x01, 0x02]); // Only 2 bytes of triggering events

        let result = TraceInformation::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid triggering events length"));
    }

    #[test]
    fn test_trace_information_unmarshal_missing_trace_depth() {
        let mut payload = vec![0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF]; // mcc_mnc + trace_id
        payload.push(2); // triggering_events_len = 2
        payload.extend(vec![0x01, 0x02]); // triggering events data
                                          // Missing trace_depth

        let result = TraceInformation::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing trace depth"));
    }

    #[test]
    fn test_trace_information_len() {
        let trace_info_minimal = TraceInformation::new([0; 3], [0; 3], vec![], 0, vec![]);
        assert_eq!(trace_info_minimal.len(), 10); // 3+3+1+0+1+1+0+1+0 = 10

        let trace_info_with_data = TraceInformation::new(
            [0; 3],
            [0; 3],
            vec![0x01, 0x02], // 2 bytes
            0,
            vec![0x10, 0x20, 0x30], // 3 bytes
        )
        .with_trace_collection_entity_ipv4(Ipv4Addr::new(192, 168, 1, 1)); // 4 bytes

        assert_eq!(trace_info_with_data.len(), 19); // 3+3+1+2+1+1+3+1+4 = 19
    }

    #[test]
    fn test_trace_information_round_trip_empty_lists() {
        let trace_info = TraceInformation::new([0xFF; 3], [0xAA; 3], vec![], 255, vec![]);

        let marshaled = trace_info.marshal();
        let unmarshaled = TraceInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(trace_info, unmarshaled);
        assert!(unmarshaled.triggering_events.is_empty());
        assert!(unmarshaled.list_of_interfaces.is_empty());
        assert_eq!(unmarshaled.ip_address_of_trace_collection_entity, None);
    }
}
