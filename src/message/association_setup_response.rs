// src/message/association_setup_response.rs

//! Association Setup Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupResponse {
    pub header: Header,
    pub cause: Ie,
    pub node_id: Ie,
    pub up_function_features: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub recovery_time_stamp: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for AssociationSetupResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        data.extend_from_slice(&self.node_id.marshal());
        if let Some(ref ie) = self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.recovery_time_stamp {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        // Update length
        let len = (data.len() - 4) as u16;
        data[2..4].copy_from_slice(&len.to_be_bytes());
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut cause = None;
        let mut node_id = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut recovery_time_stamp = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::NodeId => node_id = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
            up_function_features,
            cp_function_features,
            recovery_time_stamp,
            ies,
        })
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
            IeType::NodeId => Some(&self.node_id),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            IeType::RecoveryTimeStamp => self.recovery_time_stamp.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}
