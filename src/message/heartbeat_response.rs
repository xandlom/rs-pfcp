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
