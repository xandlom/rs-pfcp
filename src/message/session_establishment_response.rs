//! Session Establishment Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Establishment Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub fseid: Ie,
    pub created_pdr: Option<Ie>,
    pub load_control_information: Option<Ie>,
    pub overload_control_information: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionEstablishmentResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        data.extend_from_slice(&self.fseid.marshal());
        if let Some(ie) = &self.created_pdr {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.load_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.overload_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut fseid = None;
        let mut created_pdr = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::Fseid => fseid = Some(ie),
                IeType::CreatedPdr => created_pdr = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionEstablishmentResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            fseid: fseid
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE not found"))?,
            created_pdr,
            load_control_information,
            overload_control_information,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionEstablishmentResponse
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
        match ie_type {
            IeType::Cause => Some(&self.cause),
            IeType::Fseid => Some(&self.fseid),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::CreatedPdr => self.created_pdr.as_ref(),
            IeType::LoadControlInformation => self.load_control_information.as_ref(),
            IeType::OverloadControlInformation => self.overload_control_information.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}
