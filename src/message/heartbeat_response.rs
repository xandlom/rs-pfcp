//! Heartbeat Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Heartbeat Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatResponse {
    pub header: Header,
    pub recovery_time_stamp: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl HeartbeatResponse {
    /// Creates a new Heartbeat Response message.
    pub fn new(seq: u32, ts: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = 0;
        if let Some(ref ie) = ts {
            payload_len += ie.len();
        }
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
}

impl Message for HeartbeatResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        if let Some(ref ie) = self.recovery_time_stamp {
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

        Ok(HeartbeatResponse {
            header,
            recovery_time_stamp,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::HeartbeatResponse
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
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

/// Builder for HeartbeatResponse message.
#[derive(Debug, Default)]
pub struct HeartbeatResponseBuilder {
    sequence: u32,
    recovery_time_stamp: Option<Ie>,
    ies: Vec<Ie>,
}

impl HeartbeatResponseBuilder {
    /// Creates a new HeartbeatResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
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
    pub fn build(self) -> HeartbeatResponse {
        HeartbeatResponse::new(self.sequence, self.recovery_time_stamp, self.ies)
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
        let response = HeartbeatResponseBuilder::new(12345).build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::HeartbeatResponse);
        assert!(response.recovery_time_stamp.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_heartbeat_response_builder_with_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let recovery_ts = RecoveryTimeStamp::new(timestamp);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.recovery_time_stamp, Some(recovery_ie));
    }

    #[test]
    fn test_heartbeat_response_builder_with_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x03]);

        let response = HeartbeatResponseBuilder::new(12345)
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.ies.len(), 3);
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

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.recovery_time_stamp, Some(recovery_ie));
        assert_eq!(response.ies.len(), 1);
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
}
