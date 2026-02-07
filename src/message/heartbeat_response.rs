//! Heartbeat Response message.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Represents a Heartbeat Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatResponse {
    header: Header,
    recovery_time_stamp: Ie, // M - 3GPP TS 29.244 Table 7.4.2.2-1 - IE Type 96
    ies: Vec<Ie>,
}

impl HeartbeatResponse {
    /// Creates a new Heartbeat Response message.
    pub fn new(seq: impl Into<SequenceNumber>, ts: Ie, ies: Vec<Ie>) -> Self {
        let mut payload_len = ts.len();
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::HeartbeatResponse, false, 0, seq);
        header.length = 4 + payload_len;

        HeartbeatResponse {
            header,
            recovery_time_stamp: ts,
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
    /// use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
    ///
    /// let response = HeartbeatResponseBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .build();
    ///
    /// let ts = response.recovery_time_stamp().unwrap();
    /// ```
    pub fn recovery_time_stamp(
        &self,
    ) -> Result<crate::ie::recovery_time_stamp::RecoveryTimeStamp, PfcpError> {
        crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(
            &self.recovery_time_stamp.payload,
        )
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
}

impl Message for HeartbeatResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.recovery_time_stamp.marshal_into(buf);
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.recovery_time_stamp.len() as usize;
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut recovery_time_stamp = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        // Validate mandatory IE is present per 3GPP TS 29.244 Table 7.4.2.2-1
        let recovery_time_stamp = recovery_time_stamp.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::RecoveryTimeStamp,
            message_type: Some(MsgType::HeartbeatResponse),
            parent_ie: None,
        })?;

        Ok(HeartbeatResponse {
            header,
            recovery_time_stamp,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::HeartbeatResponse
    }

    fn seid(&self) -> Option<Seid> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> SequenceNumber {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: SequenceNumber) {
        self.header.sequence_number = seq;
    }

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::RecoveryTimeStamp => IeIter::single(Some(&self.recovery_time_stamp), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = Vec::new();
        result.push(&self.recovery_time_stamp);
        result.extend(self.ies.iter());
        result
    }
}

/// Builder for HeartbeatResponse message.
#[derive(Debug, Default)]
pub struct HeartbeatResponseBuilder {
    sequence: SequenceNumber,
    recovery_time_stamp: Option<Ie>,
    ies: Vec<Ie>,
}

impl HeartbeatResponseBuilder {
    /// Creates a new HeartbeatResponse builder.
    pub fn new(sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            sequence: sequence.into(),
            recovery_time_stamp: None,
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
    /// use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
    ///
    /// let response = HeartbeatResponseBuilder::new(1)
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
    pub fn recovery_time_stamp_ie(mut self, recovery_time_stamp: Ie) -> Self {
        self.recovery_time_stamp = Some(recovery_time_stamp);
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

    /// Builds the HeartbeatResponse message.
    ///
    /// # Panics
    ///
    /// Panics if the mandatory recovery_time_stamp is not set.
    /// Per 3GPP TS 29.244 Table 7.4.2.2-1, Recovery Time Stamp is mandatory.
    pub fn build(self) -> HeartbeatResponse {
        let recovery_time_stamp = self.recovery_time_stamp.expect(
            "HeartbeatResponse requires recovery_time_stamp (mandatory per 3GPP TS 29.244 Table 7.4.2.2-1)"
        );
        HeartbeatResponse::new(self.sequence, recovery_time_stamp, self.ies)
    }

    /// Builds the HeartbeatResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
    ///
    /// let bytes = HeartbeatResponseBuilder::new(1)
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
    use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
    use std::time::SystemTime;

    #[test]
    fn test_heartbeat_response_builder_minimal() {
        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::HeartbeatResponse);
        assert_eq!(
            response.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(response.additional_ies().is_empty());
    }

    #[test]
    fn test_heartbeat_response_builder_with_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 12345);
        assert_eq!(response.recovery_time_stamp_ie(), &recovery_ie);
    }

    #[test]
    fn test_heartbeat_response_builder_with_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x03]);

        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp(SystemTime::now())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.additional_ies().len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_heartbeat_response_builder_full() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let additional_ie = Ie::new(IeType::Unknown, vec![0x01, 0x02, 0x03]);

        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 12345);
        assert_eq!(response.recovery_time_stamp_ie(), &recovery_ie);
        assert_eq!(response.additional_ies().len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_heartbeat_response_roundtrip_via_builder() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let original = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_convenience_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let response = HeartbeatResponseBuilder::new(1000)
            .recovery_time_stamp(timestamp)
            .build();

        assert_eq!(*response.sequence(), 1000);
        assert_eq!(
            response.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );

        // Verify the IE was created correctly
        let ie = response.recovery_time_stamp_ie();
        assert_eq!(ie.ie_type, IeType::RecoveryTimeStamp);

        // Verify it can be unmarshaled
        let recovered =
            RecoveryTimeStamp::unmarshal(&response.recovery_time_stamp_ie().payload).unwrap();
        // SystemTime comparison with tolerance (within 1 second)
        let duration = timestamp
            .duration_since(recovered.timestamp)
            .unwrap_or_else(|e| e.duration());
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_builder_marshal_convenience() {
        let bytes = HeartbeatResponseBuilder::new(2000)
            .recovery_time_stamp(SystemTime::now())
            .marshal();

        assert!(!bytes.is_empty());
        // Should be able to unmarshal the bytes
        let unmarshaled = HeartbeatResponse::unmarshal(&bytes).unwrap();
        assert_eq!(*unmarshaled.sequence(), 2000);
    }

    #[test]
    fn test_ies_recovery_timestamp() {
        let response = HeartbeatResponseBuilder::new(3000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = response.ies(IeType::RecoveryTimeStamp).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_ies_in_additional_ies() {
        let custom_ie = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA, 0xBB]);
        let response = HeartbeatResponseBuilder::new(4000)
            .recovery_time_stamp(SystemTime::now())
            .ie(custom_ie.clone())
            .build();

        let found = response.ies(IeType::UserPlaneIpResourceInformation).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &custom_ie);
    }

    #[test]
    fn test_ies_not_found() {
        let response = HeartbeatResponseBuilder::new(5000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        // Recovery timestamp will be found, so test for a different IE
        let found = response.ies(IeType::SourceIpAddress).next();
        assert!(found.is_none());
    }

    #[test]
    fn test_set_sequence() {
        let mut response = HeartbeatResponseBuilder::new(6000)
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*response.sequence(), 6000);
        response.set_sequence(9999.into());
        assert_eq!(*response.sequence(), 9999);
    }

    #[test]
    fn test_seid_should_be_none() {
        // Heartbeat messages never have SEID
        let response = HeartbeatResponseBuilder::new(7000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        assert!(response.seid().is_none());
    }

    #[test]
    fn test_recovery_timestamp_unix_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        let response = HeartbeatResponseBuilder::new(8000)
            .recovery_time_stamp(epoch)
            .build();

        let marshaled = response.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 8000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
    }

    #[test]
    fn test_recovery_timestamp_future() {
        use std::time::Duration;
        let future = SystemTime::now() + Duration::from_secs(3600 * 24 * 365); // 1 year from now
        let response = HeartbeatResponseBuilder::new(9000)
            .recovery_time_stamp(future)
            .build();

        let marshaled = response.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 9000);
    }

    #[test]
    fn test_with_multiple_additional_ies() {
        let ie1 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x01]);
        let ie2 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x02]);
        let ie3 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x03]);

        let response = HeartbeatResponseBuilder::new(10000)
            .recovery_time_stamp(SystemTime::now())
            .ie(ie1.clone())
            .ie(ie2.clone())
            .ie(ie3.clone())
            .build();

        assert_eq!(*response.sequence(), 10000);
        assert_eq!(
            response.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert_eq!(response.additional_ies().len(), 3);

        // Round trip
        let marshaled = response.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 10000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert_eq!(unmarshaled.additional_ies().len(), 3);
    }

    #[test]
    fn test_unmarshal_minimal_message() {
        // Minimal message with only mandatory recovery_time_stamp
        let response = HeartbeatResponseBuilder::new(11000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let marshaled = response.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(*unmarshaled.sequence(), 11000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert!(unmarshaled.additional_ies().is_empty());
    }

    #[test]
    fn test_header_length_calculation() {
        // Minimal message (with mandatory recovery_time_stamp)
        let minimal = HeartbeatResponseBuilder::new(12000)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let minimal_bytes = minimal.marshal();
        // Header overhead + recovery timestamp IE
        assert!(minimal.header.length > 4);

        // With recovery timestamp + additional IE
        let with_ie = HeartbeatResponseBuilder::new(13000)
            .recovery_time_stamp(SystemTime::now())
            .ie(Ie::new(
                IeType::UserPlaneIpResourceInformation,
                vec![0x01, 0x02],
            ))
            .build();
        let with_ie_bytes = with_ie.marshal();
        assert!(with_ie.header.length > minimal.header.length);

        // Verify unmarshal works
        HeartbeatResponse::unmarshal(&minimal_bytes).unwrap();
        HeartbeatResponse::unmarshal(&with_ie_bytes).unwrap();
    }

    #[test]
    fn test_builder_method_chaining() {
        let response = HeartbeatResponseBuilder::new(14000)
            .recovery_time_stamp(SystemTime::now())
            .ie(Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA]))
            .ies(vec![
                Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xBB]),
                Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xCC]),
            ])
            .build();

        assert_eq!(*response.sequence(), 14000);
        assert_eq!(
            response.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert_eq!(response.additional_ies().len(), 3);
    }

    #[test]
    fn test_multiple_roundtrips() {
        // Test that we can roundtrip multiple times without loss
        let original = HeartbeatResponseBuilder::new(15000)
            .recovery_time_stamp(SystemTime::now())
            .ie(Ie::new(
                IeType::UserPlaneIpResourceInformation,
                vec![0x12, 0x34],
            ))
            .build();

        let bytes1 = original.marshal();
        let unmarshaled1 = HeartbeatResponse::unmarshal(&bytes1).unwrap();

        let bytes2 = unmarshaled1.marshal();
        let unmarshaled2 = HeartbeatResponse::unmarshal(&bytes2).unwrap();

        let bytes3 = unmarshaled2.marshal();
        let unmarshaled3 = HeartbeatResponse::unmarshal(&bytes3).unwrap();

        // All should be identical
        assert_eq!(unmarshaled1, unmarshaled2);
        assert_eq!(unmarshaled2, unmarshaled3);
    }

    #[test]
    fn test_ergonomic_one_liner() {
        let bytes = HeartbeatResponseBuilder::new(16000)
            .recovery_time_stamp(SystemTime::now())
            .marshal();

        assert!(!bytes.is_empty());
        assert!(HeartbeatResponse::unmarshal(&bytes).is_ok());
    }

    #[test]
    fn test_full_roundtrip_with_all_features() {
        let ie1 = Ie::new(
            IeType::UserPlaneIpResourceInformation,
            vec![0x01, 0x02, 0x03],
        );
        let ie2 = Ie::new(
            IeType::UserPlaneIpResourceInformation,
            vec![0x04, 0x05, 0x06],
        );

        let original = HeartbeatResponseBuilder::new(17000)
            .recovery_time_stamp(SystemTime::now())
            .ie(ie1)
            .ie(ie2)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = HeartbeatResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(*unmarshaled.sequence(), 17000);
        assert_eq!(
            unmarshaled.recovery_time_stamp_ie().ie_type,
            IeType::RecoveryTimeStamp
        );
        assert_eq!(unmarshaled.additional_ies().len(), 2);
    }

    #[test]
    fn test_unmarshal_missing_mandatory_recovery_timestamp() {
        // Create a message without recovery timestamp - should fail
        use crate::message::header::Header;

        let header = Header::new(MsgType::HeartbeatResponse, false, 0, 18000);
        let marshaled = header.marshal();

        // Unmarshaling should fail because recovery_time_stamp is mandatory
        let result = HeartbeatResponse::unmarshal(&marshaled);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::RecoveryTimeStamp);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    #[should_panic(expected = "HeartbeatResponse requires recovery_time_stamp")]
    fn test_builder_without_mandatory_field_panics() {
        // Builder should panic if recovery_time_stamp is not set
        HeartbeatResponseBuilder::new(19000).build();
    }
}
