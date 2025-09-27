//! Heartbeat Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Heartbeat Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRequest {
    pub header: Header,
    pub recovery_time_stamp: Option<Ie>,
    pub source_ip_address: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl HeartbeatRequest {
    /// Creates a new Heartbeat Request message.
    pub fn new(seq: u32, ts: Option<Ie>, ip: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = 0;
        if let Some(ref ie) = ts {
            payload_len += ie.len();
        }
        if let Some(ref ie) = ip {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::HeartbeatRequest, false, 0, seq);
        header.length = 4 + payload_len;

        HeartbeatRequest {
            header,
            recovery_time_stamp: ts,
            source_ip_address: ip,
            ies,
        }
    }
}

impl Message for HeartbeatRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        if let Some(ref ie) = self.recovery_time_stamp {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.source_ip_address {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut recovery_time_stamp = None;
        let mut source_ip_address = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                IeType::SourceIpAddress => source_ip_address = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(HeartbeatRequest {
            header,
            recovery_time_stamp,
            source_ip_address,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::HeartbeatRequest
    }

    fn seid(&self) -> Option<u64> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: u32) {
        self.header.sequence_number = seq;
    }

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        if self
            .recovery_time_stamp
            .as_ref()
            .is_some_and(|ie| ie.ie_type == ie_type)
        {
            return self.recovery_time_stamp.as_ref();
        }
        if self
            .source_ip_address
            .as_ref()
            .is_some_and(|ie| ie.ie_type == ie_type)
        {
            return self.source_ip_address.as_ref();
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

/// Builder for HeartbeatRequest message.
#[derive(Debug, Default)]
pub struct HeartbeatRequestBuilder {
    sequence: u32,
    recovery_time_stamp: Option<Ie>,
    source_ip_address: Option<Ie>,
    ies: Vec<Ie>,
}

impl HeartbeatRequestBuilder {
    /// Creates a new HeartbeatRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            recovery_time_stamp: None,
            source_ip_address: None,
            ies: Vec::new(),
        }
    }

    /// Sets the recovery time stamp IE.
    pub fn recovery_time_stamp(mut self, recovery_time_stamp: Ie) -> Self {
        self.recovery_time_stamp = Some(recovery_time_stamp);
        self
    }

    /// Sets the source IP address IE.
    pub fn source_ip_address(mut self, source_ip_address: Ie) -> Self {
        self.source_ip_address = Some(source_ip_address);
        self
    }

    /// Adds an additional IE.
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple IEs.
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the HeartbeatRequest message.
    pub fn build(self) -> HeartbeatRequest {
        HeartbeatRequest::new(
            self.sequence,
            self.recovery_time_stamp,
            self.source_ip_address,
            self.ies,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{recovery_time_stamp::RecoveryTimeStamp, source_ip_address::SourceIpAddress};
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    #[test]
    fn test_heartbeat_request_builder_minimal() {
        let request = HeartbeatRequestBuilder::new(12345).build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.msg_type(), MsgType::HeartbeatRequest);
        assert!(request.recovery_time_stamp.is_none());
        assert!(request.source_ip_address.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_heartbeat_request_builder_with_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(recovery_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.recovery_time_stamp, Some(recovery_ie));
        assert!(request.source_ip_address.is_none());
    }

    #[test]
    fn test_heartbeat_request_builder_with_source_ip() {
        let ip = SourceIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let ip_ie = Ie::new(IeType::SourceIpAddress, ip.marshal());

        let request = HeartbeatRequestBuilder::new(12345)
            .source_ip_address(ip_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert!(request.recovery_time_stamp.is_none());
        assert_eq!(request.source_ip_address, Some(ip_ie));
    }

    #[test]
    fn test_heartbeat_request_builder_full() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let ip = SourceIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let ip_ie = Ie::new(IeType::SourceIpAddress, ip.marshal());

        let additional_ie = Ie::new(IeType::Unknown, vec![0x01, 0x02, 0x03]);

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(recovery_ie.clone())
            .source_ip_address(ip_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.recovery_time_stamp, Some(recovery_ie));
        assert_eq!(request.source_ip_address, Some(ip_ie));
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_heartbeat_request_builder_with_multiple_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x03]);

        let request = HeartbeatRequestBuilder::new(12345)
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_heartbeat_request_roundtrip_via_builder() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let original = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(recovery_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
