//! Heartbeat Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Heartbeat Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRequest {
    header: Header,
    recovery_time_stamp: Ie, // M - 3GPP TS 29.244 Table 7.4.2.1-1 - IE Type 96
    source_ip_address: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.2.1-1 - IE Type 192 - When NAT is deployed
    ies: Vec<Ie>,
}

impl HeartbeatRequest {
    /// Creates a new Heartbeat Request message.
    pub fn new(seq: u32, ts: Ie, ip: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = ts.len();
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

    // Typed accessors (recommended API)

    /// Returns the recovery time stamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    ///
    /// let request = HeartbeatRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .build();
    ///
    /// let ts = request.recovery_time_stamp().unwrap();
    /// ```
    pub fn recovery_time_stamp(&self) -> Result<crate::ie::recovery_time_stamp::RecoveryTimeStamp, io::Error> {
        crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(&self.recovery_time_stamp.payload)
    }

    /// Returns the source IP address if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    ///
    /// let request = HeartbeatRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .source_ip_address(Ipv4Addr::new(192, 168, 1, 1))
    ///     .build();
    ///
    /// let source_ip = request.source_ip_address().unwrap().unwrap();
    /// ```
    pub fn source_ip_address(&self) -> Option<Result<crate::ie::source_ip_address::SourceIpAddress, io::Error>> {
        self.source_ip_address.as_ref()
            .map(|ie| crate::ie::source_ip_address::SourceIpAddress::unmarshal(&ie.payload))
    }

    /// Returns a slice of additional IEs.
    pub fn additional_ies(&self) -> &[Ie] {
        &self.ies
    }

    // Raw IE accessors (compatibility layer)

    /// Returns the raw recovery time stamp IE.
    pub fn recovery_time_stamp_ie(&self) -> &Ie {
        &self.recovery_time_stamp
    }

    /// Returns the raw source IP address IE if present.
    pub fn source_ip_address_ie(&self) -> Option<&Ie> {
        self.source_ip_address.as_ref()
    }
}

impl Message for HeartbeatRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.recovery_time_stamp.marshal());
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

        // Validate mandatory IE is present per 3GPP TS 29.244 Table 7.4.2.1-1
        let recovery_time_stamp = recovery_time_stamp.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "HeartbeatRequest: Missing mandatory Recovery Time Stamp IE (3GPP TS 29.244 Table 7.4.2.1-1)",
            )
        })?;

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
        if self.recovery_time_stamp.ie_type == ie_type {
            return Some(&self.recovery_time_stamp);
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

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = Vec::new();
        result.push(&self.recovery_time_stamp);
        if let Some(ref ie) = self.source_ip_address {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
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

    /// Sets the recovery time stamp from a `SystemTime`.
    ///
    /// This is an ergonomic method that automatically converts the `SystemTime`
    /// to a `RecoveryTimeStamp` IE. For more control, use [`recovery_time_stamp_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    ///
    /// let request = HeartbeatRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .build();
    /// ```
    ///
    /// [`recovery_time_stamp_ie`]: #method.recovery_time_stamp_ie
    pub fn recovery_time_stamp(mut self, timestamp: std::time::SystemTime) -> Self {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(Ie::new(IeType::RecoveryTimeStamp, ts.marshal().to_vec()));
        self
    }

    /// Sets the recovery time stamp IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`recovery_time_stamp`] which accepts a `SystemTime` directly.
    ///
    /// [`recovery_time_stamp`]: #method.recovery_time_stamp
    pub fn recovery_time_stamp_ie(mut self, ie: Ie) -> Self {
        self.recovery_time_stamp = Some(ie);
        self
    }

    /// Sets the source IP address from an IP address.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For more control, use
    /// [`source_ip_address_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    ///
    /// let request = HeartbeatRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .source_ip_address(Ipv4Addr::new(192, 168, 1, 1))
    ///     .build();
    /// ```
    ///
    /// [`source_ip_address_ie`]: #method.source_ip_address_ie
    pub fn source_ip_address<T>(mut self, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::source_ip_address::SourceIpAddress;
        let ip_addr = ip_addr.into();
        let source_ip = match ip_addr {
            std::net::IpAddr::V4(v4) => SourceIpAddress::new(Some(v4), None),
            std::net::IpAddr::V6(v6) => SourceIpAddress::new(None, Some(v6)),
        };
        self.source_ip_address = Some(source_ip.to_ie());
        self
    }

    /// Sets the source IP address IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`source_ip_address`] which accepts IP addresses directly.
    ///
    /// [`source_ip_address`]: #method.source_ip_address
    pub fn source_ip_address_ie(mut self, ie: Ie) -> Self {
        self.source_ip_address = Some(ie);
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
    ///
    /// # Panics
    ///
    /// Panics if the mandatory recovery_time_stamp is not set.
    /// Per 3GPP TS 29.244 Table 7.4.2.1-1, Recovery Time Stamp is mandatory.
    pub fn build(self) -> HeartbeatRequest {
        let recovery_time_stamp = self.recovery_time_stamp.expect(
            "HeartbeatRequest requires recovery_time_stamp (mandatory per 3GPP TS 29.244 Table 7.4.2.1-1)"
        );
        HeartbeatRequest::new(
            self.sequence,
            recovery_time_stamp,
            self.source_ip_address,
            self.ies,
        )
    }

    /// Builds the HeartbeatRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    ///
    /// let bytes = HeartbeatRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{recovery_time_stamp::RecoveryTimeStamp, source_ip_address::SourceIpAddress};
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::time::SystemTime;

    #[test]
    fn test_heartbeat_request_builder_minimal() {
        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.msg_type(), MsgType::HeartbeatRequest);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(request.source_ip_address_ie().is_none());
        assert!(request.additional_ies().is_empty());
    }

    #[test]
    fn test_heartbeat_request_builder_with_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.recovery_time_stamp_ie(), &recovery_ie);
        assert!(request.source_ip_address_ie().is_none());
    }

    #[test]
    fn test_heartbeat_request_builder_with_source_ip() {
        let ip = SourceIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let ip_ie = Ie::new(IeType::SourceIpAddress, ip.marshal());

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address_ie(ip_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert_eq!(request.source_ip_address_ie(), Some(&ip_ie));
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
            .recovery_time_stamp_ie(recovery_ie.clone())
            .source_ip_address_ie(ip_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.recovery_time_stamp, recovery_ie);
        assert_eq!(request.source_ip_address_ie(), Some(&ip_ie));
        assert_eq!(request.additional_ies().len(), 1);
        assert_eq!(request.additional_ies()[0], additional_ie);
    }

    #[test]
    fn test_heartbeat_request_builder_with_multiple_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x03]);

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.additional_ies().len(), 3);
        assert_eq!(request.additional_ies()[0], ie1);
        assert_eq!(request.additional_ies()[1], ie2);
        assert_eq!(request.additional_ies()[2], ie3);
    }

    #[test]
    fn test_heartbeat_request_roundtrip_via_builder() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let original = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    // Ergonomic API tests
    #[test]
    fn test_heartbeat_request_ergonomic_timestamp() {
        let timestamp = SystemTime::now();

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(timestamp)
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );

        // Verify the IE was created correctly
        let ie = request.recovery_time_stamp_ie();
        assert_eq!(ie.ie_type, IeType::RecoveryTimeStamp);

        // Verify it can be unmarshaled
        let recovered = RecoveryTimeStamp::unmarshal(&request.recovery_time_stamp_ie().payload).unwrap();
        // SystemTime comparison with tolerance (within 1 second)
        let duration = timestamp
            .duration_since(recovered.timestamp)
            .unwrap_or_else(|e| e.duration());
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_heartbeat_request_ergonomic_ipv4() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(ipv4)
            .build();

        assert!(request.source_ip_address_ie().is_some());
        let ie = request.source_ip_address_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::SourceIpAddress);

        // Verify it unmarshals correctly
        let source_ip = SourceIpAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, None);
    }

    #[test]
    fn test_heartbeat_request_ergonomic_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 1);

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(ipv6)
            .build();

        assert!(request.source_ip_address_ie().is_some());
        let ie = request.source_ip_address_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::SourceIpAddress);

        // Verify the IE contains flags + IPv6 address bytes
        assert_eq!(ie.payload.len(), 17); // 1 byte flags + 16 bytes IPv6
        assert_eq!(ie.payload[0], 0x01); // V6 flag only

        // Verify round-trip now works correctly
        let source_ip = SourceIpAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(source_ip.ipv6, Some(ipv6));
        assert_eq!(source_ip.ipv4, None);
        assert!(source_ip.v6);
        assert!(!source_ip.v4);
    }

    #[test]
    fn test_heartbeat_request_ergonomic_full_chain() {
        let bytes = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv4Addr::new(192, 168, 1, 1))
            .build()
            .marshal();

        // Should be able to unmarshal
        let request = HeartbeatRequest::unmarshal(&bytes).unwrap();
        assert_eq!(request.sequence(), 12345);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(request.source_ip_address_ie().is_some());
    }

    #[test]
    fn test_heartbeat_request_ergonomic_marshal_method() {
        // Test the .marshal() convenience method
        let bytes = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .marshal();

        // Should produce valid bytes
        let request = HeartbeatRequest::unmarshal(&bytes).unwrap();
        assert_eq!(request.sequence(), 12345);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
    }

    #[test]
    fn test_heartbeat_request_ergonomic_one_liner() {
        // The ultimate ergonomic test - everything in one line
        let bytes = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .marshal();

        assert!(!bytes.is_empty());
        assert!(HeartbeatRequest::unmarshal(&bytes).is_ok());
    }

    #[test]
    fn test_find_ie_recovery_timestamp() {
        let request = HeartbeatRequestBuilder::new(1000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = request.find_ie(IeType::RecoveryTimeStamp);
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_find_ie_source_ip_address() {
        let request = HeartbeatRequestBuilder::new(2000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv4Addr::new(10, 0, 0, 1))
            .build();

        let found = request.find_ie(IeType::SourceIpAddress);
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::SourceIpAddress);
    }

    #[test]
    fn test_find_ie_in_additional_ies() {
        let custom_ie = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA, 0xBB]);
        let request = HeartbeatRequestBuilder::new(3000)
            .recovery_time_stamp(SystemTime::now())
            .ie(custom_ie.clone())
            .build();

        let found = request.find_ie(IeType::UserPlaneIpResourceInformation);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &custom_ie);
    }

    #[test]
    fn test_find_ie_not_found() {
        let request = HeartbeatRequestBuilder::new(4000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        // Recovery timestamp will be found, so test for a different IE
        let found = request.find_ie(IeType::SourceIpAddress);
        assert!(found.is_none());
    }

    #[test]
    fn test_set_sequence() {
        let mut request = HeartbeatRequestBuilder::new(5000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(request.sequence(), 5000);
        request.set_sequence(9999);
        assert_eq!(request.sequence(), 9999);
    }

    #[test]
    fn test_seid_should_be_none() {
        // Heartbeat messages never have SEID
        let request = HeartbeatRequestBuilder::new(6000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        assert!(request.seid().is_none());
    }

    #[test]
    fn test_recovery_timestamp_unix_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        let request = HeartbeatRequestBuilder::new(7000)
            .recovery_time_stamp(epoch)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.sequence(), 7000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
    }

    #[test]
    fn test_recovery_timestamp_future() {
        use std::time::Duration;
        let future = SystemTime::now() + Duration::from_secs(3600 * 24 * 365); // 1 year from now
        let request = HeartbeatRequestBuilder::new(8000)
            .recovery_time_stamp(future)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.sequence(), 8000);
    }

    #[test]
    fn test_source_ip_ipv4_roundtrip() {
        let ipv4 = Ipv4Addr::new(192, 168, 50, 50);
        let request = HeartbeatRequestBuilder::new(9000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(ipv4)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.sequence(), 9000);
        assert!(unmarshaled.source_ip_address_ie().is_some());

        let ie = unmarshaled.source_ip_address_ie().unwrap();
        let source_ip = SourceIpAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(source_ip.ipv4, Some(ipv4));
    }

    #[test]
    fn test_source_ip_ipv6_roundtrip() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let request = HeartbeatRequestBuilder::new(10000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(ipv6)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.sequence(), 10000);
        assert!(unmarshaled.source_ip_address_ie().is_some());

        let ie = unmarshaled.source_ip_address_ie().unwrap();
        let source_ip = SourceIpAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(source_ip.ipv6, Some(ipv6));
    }

    #[test]
    fn test_all_optional_ies_combined() {
        let request = HeartbeatRequestBuilder::new(11000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv4Addr::new(10, 1, 1, 1))
            .ie(Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x01]))
            .ie(Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x02]))
            .build();

        assert_eq!(request.sequence(), 11000);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(request.source_ip_address_ie().is_some());
        assert_eq!(request.additional_ies().len(), 2);

        // Round trip
        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.sequence(), 11000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(unmarshaled.source_ip_address_ie().is_some());
        assert_eq!(unmarshaled.additional_ies().len(), 2);
    }

    #[test]
    fn test_unmarshal_minimal_message() {
        // Minimal message with only mandatory recovery_time_stamp
        let request = HeartbeatRequestBuilder::new(12000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let marshaled = request.marshal();
        let unmarshaled = HeartbeatRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.sequence(), 12000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(unmarshaled.source_ip_address_ie().is_none());
        assert!(unmarshaled.additional_ies().is_empty());
    }

    #[test]
    fn test_header_length_calculation() {
        // Minimal message (with mandatory recovery_time_stamp)
        let minimal = HeartbeatRequestBuilder::new(13000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let minimal_bytes = minimal.marshal();
        // Header overhead + recovery timestamp IE
        assert!(minimal.header.length > 4);

        // With recovery timestamp + source IP
        let with_ts = HeartbeatRequestBuilder::new(14000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv4Addr::new(192, 168, 1, 1))
            .build();
        let with_ts_bytes = with_ts.marshal();
        assert!(with_ts.header.length > minimal.header.length);

        // Verify unmarshal works
        HeartbeatRequest::unmarshal(&minimal_bytes).unwrap();
        HeartbeatRequest::unmarshal(&with_ts_bytes).unwrap();
    }

    #[test]
    fn test_builder_method_chaining() {
        let request = HeartbeatRequestBuilder::new(15000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv4Addr::new(10, 2, 2, 2))
            .ie(Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA]))
            .ies(vec![
                Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xBB]),
                Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xCC]),
            ])
            .build();

        assert_eq!(request.sequence(), 15000);
        assert_eq!(
            request.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(request.source_ip_address_ie().is_some());
        assert_eq!(request.additional_ies().len(), 3);
    }

    #[test]
    fn test_multiple_roundtrips() {
        // Test that we can roundtrip multiple times without loss
        let original = HeartbeatRequestBuilder::new(16000)
            .recovery_time_stamp(SystemTime::now())
            .source_ip_address(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1))
            .build();

        let bytes1 = original.marshal();
        let unmarshaled1 = HeartbeatRequest::unmarshal(&bytes1).unwrap();

        let bytes2 = unmarshaled1.marshal();
        let unmarshaled2 = HeartbeatRequest::unmarshal(&bytes2).unwrap();

        let bytes3 = unmarshaled2.marshal();
        let unmarshaled3 = HeartbeatRequest::unmarshal(&bytes3).unwrap();

        // All should be identical
        assert_eq!(unmarshaled1, unmarshaled2);
        assert_eq!(unmarshaled2, unmarshaled3);
    }

    #[test]
    fn test_unmarshal_missing_mandatory_recovery_timestamp() {
        // Create a message without recovery timestamp - should fail
        use crate::message::header::Header;

        let header = Header::new(MsgType::HeartbeatRequest, false, 0, 17000);
        let marshaled = header.marshal();

        // Unmarshaling should fail because recovery_time_stamp is mandatory
        let result = HeartbeatRequest::unmarshal(&marshaled);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    #[should_panic(expected = "HeartbeatRequest requires recovery_time_stamp")]
    fn test_builder_without_mandatory_field_panics() {
        // Builder should panic if recovery_time_stamp is not set
        HeartbeatRequestBuilder::new(18000).build();
    }
}
