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
                IeType::SourceIPAddress => source_ip_address = Some(ie),
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
