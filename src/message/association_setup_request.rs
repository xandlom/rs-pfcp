// src/message/association_setup_request.rs

//! Association Setup Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupRequest {
    pub header: Header,
    pub node_id: Ie,
    pub recovery_time_stamp: Ie,
    pub up_function_features: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub ies: Vec<Ie>, // For any other IEs
}

impl Message for AssociationSetupRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data.extend_from_slice(&self.recovery_time_stamp.marshal());
        if let Some(ie) = &self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut recovery_time_stamp = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupRequest {
            header,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
            recovery_time_stamp: recovery_time_stamp.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Recovery Time Stamp IE not found",
                )
            })?,
            up_function_features,
            cp_function_features,
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
            IeType::NodeId => Some(&self.node_id),
            IeType::RecoveryTimeStamp => Some(&self.recovery_time_stamp),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

impl AssociationSetupRequest {
    pub fn new(
        seq: u32,
        node_id: Ie,
        recovery_time_stamp: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + recovery_time_stamp.len();
        if let Some(ie) = &up_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_function_features {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        let mut header = Header::new(MsgType::AssociationSetupRequest, false, 0, seq);
        header.length = payload_len + (header.len() - 4);
        AssociationSetupRequest {
            header,
            node_id,
            recovery_time_stamp,
            up_function_features,
            cp_function_features,
            ies,
        }
    }
}
