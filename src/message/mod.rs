//! PFCP messages and their components.

pub mod association_release_request;
pub mod association_release_response;
pub mod association_setup_request;
pub mod association_setup_response;
pub mod association_update_request;
pub mod display;
pub mod header;
pub mod heartbeat_request;
pub mod heartbeat_response;
pub mod pfd_management_request;
pub mod pfd_management_response;
pub mod session_deletion_request;
pub mod session_deletion_response;
pub mod session_establishment_request;
pub mod session_establishment_response;
pub mod session_modification_request;
pub mod session_modification_response;
pub mod session_report_request;
pub mod session_report_response;

use crate::ie::Ie;
use crate::message::association_release_request::AssociationReleaseRequest;
use crate::message::association_release_response::AssociationReleaseResponse;
use crate::message::association_setup_request::AssociationSetupRequest;
use crate::message::association_setup_response::AssociationSetupResponse;
use crate::message::association_update_request::AssociationUpdateRequest;
use crate::message::heartbeat_request::HeartbeatRequest;
use crate::message::heartbeat_response::HeartbeatResponse;
use crate::message::pfd_management_request::PfdManagementRequest;
use crate::message::pfd_management_response::PfdManagementResponse;
use crate::message::session_deletion_request::SessionDeletionRequest;
use crate::message::session_deletion_response::SessionDeletionResponse;
use crate::message::session_establishment_request::SessionEstablishmentRequest;
use crate::message::session_establishment_response::SessionEstablishmentResponse;
use crate::message::session_modification_request::SessionModificationRequest;
use crate::message::session_modification_response::SessionModificationResponse;
use crate::message::session_report_request::SessionReportRequest;
use crate::message::session_report_response::SessionReportResponse;
use std::io;

// Message Type definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MsgType {
    HeartbeatRequest = 1,
    HeartbeatResponse = 2,
    PfdManagementRequest = 3,
    PfdManagementResponse = 4,
    AssociationSetupRequest = 5,
    AssociationSetupResponse = 6,
    AssociationUpdateRequest = 7,
    AssociationUpdateResponse = 8,
    AssociationReleaseRequest = 9,
    AssociationReleaseResponse = 10,
    VersionNotSupportedResponse = 11,
    NodeReportRequest = 12,
    NodeReportResponse = 13,
    SessionSetDeletionRequest = 14,
    SessionSetDeletionResponse = 15,
    SessionEstablishmentRequest = 50,
    SessionEstablishmentResponse = 51,
    SessionModificationRequest = 52,
    SessionModificationResponse = 53,
    SessionDeletionRequest = 54,
    SessionDeletionResponse = 55,
    SessionReportRequest = 56,
    SessionReportResponse = 57,
    Unknown,
}

impl From<u8> for MsgType {
    fn from(v: u8) -> Self {
        match v {
            1 => MsgType::HeartbeatRequest,
            2 => MsgType::HeartbeatResponse,
            3 => MsgType::PfdManagementRequest,
            4 => MsgType::PfdManagementResponse,
            5 => MsgType::AssociationSetupRequest,
            6 => MsgType::AssociationSetupResponse,
            7 => MsgType::AssociationUpdateRequest,
            8 => MsgType::AssociationUpdateResponse,
            9 => MsgType::AssociationReleaseRequest,
            10 => MsgType::AssociationReleaseResponse,
            11 => MsgType::VersionNotSupportedResponse,
            12 => MsgType::NodeReportRequest,
            13 => MsgType::NodeReportResponse,
            14 => MsgType::SessionSetDeletionRequest,
            15 => MsgType::SessionSetDeletionResponse,
            50 => MsgType::SessionEstablishmentRequest,
            51 => MsgType::SessionEstablishmentResponse,
            52 => MsgType::SessionModificationRequest,
            53 => MsgType::SessionModificationResponse,
            54 => MsgType::SessionDeletionRequest,
            55 => MsgType::SessionDeletionResponse,
            56 => MsgType::SessionReportRequest,
            57 => MsgType::SessionReportResponse,
            _ => MsgType::Unknown,
        }
    }
}

/// A trait representing a PFCP message.
pub trait Message {
    fn marshal(&self) -> Vec<u8>;
    fn unmarshal(data: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized;
    fn msg_type(&self) -> MsgType;
    fn msg_name(&self) -> String {
        format!("{:?}", self.msg_type())
    }
    fn version(&self) -> u8 {
        1
    }
    fn seid(&self) -> Option<u64>;
    fn sequence(&self) -> u32;
    fn set_sequence(&mut self, seq: u32);
    fn find_ie(&self, ie_type: crate::ie::IeType) -> Option<&Ie>;
}

// A generic message for unknown message types.
pub struct Generic {
    header: header::Header,
    ies: Vec<Ie>,
}

impl Message for Generic {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = header::Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;
        let mut ies = Vec::new();
        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let ie_len = ie.len() as usize;
            ies.push(ie);
            cursor += ie_len;
        }
        Ok(Generic { header, ies })
    }

    fn msg_type(&self) -> MsgType {
        self.header.message_type
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

    fn find_ie(&self, ie_type: crate::ie::IeType) -> Option<&Ie> {
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

// A simple parse function. In a real implementation, this would be more complex.
pub fn parse(data: &[u8]) -> Result<Box<dyn Message>, io::Error> {
    let header = header::Header::unmarshal(data)?;
    match header.message_type {
        MsgType::HeartbeatRequest => Ok(Box::new(HeartbeatRequest::unmarshal(data)?)),
        MsgType::HeartbeatResponse => Ok(Box::new(HeartbeatResponse::unmarshal(data)?)),
        MsgType::PfdManagementRequest => Ok(Box::new(PfdManagementRequest::unmarshal(data)?)),
        MsgType::PfdManagementResponse => Ok(Box::new(PfdManagementResponse::unmarshal(data)?)),
        MsgType::AssociationSetupRequest => Ok(Box::new(AssociationSetupRequest::unmarshal(data)?)),
        MsgType::AssociationSetupResponse => {
            Ok(Box::new(AssociationSetupResponse::unmarshal(data)?))
        }
        MsgType::AssociationUpdateRequest => {
            Ok(Box::new(AssociationUpdateRequest::unmarshal(data)?))
        }
        MsgType::AssociationReleaseRequest => {
            Ok(Box::new(AssociationReleaseRequest::unmarshal(data)?))
        }
        MsgType::AssociationReleaseResponse => {
            Ok(Box::new(AssociationReleaseResponse::unmarshal(data)?))
        }
        MsgType::SessionEstablishmentRequest => {
            Ok(Box::new(SessionEstablishmentRequest::unmarshal(data)?))
        }
        MsgType::SessionDeletionRequest => Ok(Box::new(SessionDeletionRequest::unmarshal(data)?)),
        MsgType::SessionDeletionResponse => Ok(Box::new(SessionDeletionResponse::unmarshal(data)?)),
        MsgType::SessionModificationRequest => {
            Ok(Box::new(SessionModificationRequest::unmarshal(data)?))
        }
        MsgType::SessionModificationResponse => {
            Ok(Box::new(SessionModificationResponse::unmarshal(data)?))
        }
        MsgType::SessionEstablishmentResponse => {
            Ok(Box::new(SessionEstablishmentResponse::unmarshal(data)?))
        }
        MsgType::SessionReportRequest => Ok(Box::new(SessionReportRequest::unmarshal(data)?)),
        MsgType::SessionReportResponse => Ok(Box::new(SessionReportResponse::unmarshal(data)?)),
        _ => Ok(Box::new(Generic::unmarshal(data)?)),
    }
}
