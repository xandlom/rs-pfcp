// src/message/association_update_response.rs

//! Association Update Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationUpdateResponse {
    pub header: Header,
    pub node_id: Ie,
    pub cause: Ie,
    pub up_function_features: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl AssociationUpdateResponse {
    /// Creates a new Association Update Response message.
    pub fn new(
        seq: u32,
        node_id: Ie,
        cause: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = up_function_features {
            payload_len += ie.len();
        }
        if let Some(ref ie) = cp_function_features {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::AssociationUpdateResponse, false, 0, seq);
        header.length = 4 + payload_len;

        AssociationUpdateResponse {
            header,
            node_id,
            cause,
            up_function_features,
            cp_function_features,
            ies: Vec::new(),
        }
    }
}

impl Message for AssociationUpdateResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationUpdateResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ref ie) = self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.cp_function_features {
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
        let mut node_id = None;
        let mut cause = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Cause => cause = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let node_id = node_id.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE")
        })?;
        let cause = cause.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing Cause IE")
        })?;

        Ok(AssociationUpdateResponse {
            header,
            node_id,
            cause,
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
            IeType::Cause => Some(&self.cause),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_update_response_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original = AssociationUpdateResponse::new(123, node_id_ie, cause_ie, None, None);
        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original.header.message_type, unmarshaled.header.message_type);
        assert_eq!(original.header.sequence_number, unmarshaled.header.sequence_number);
        assert_eq!(original.node_id, unmarshaled.node_id);
        assert_eq!(original.cause, unmarshaled.cause);
        assert_eq!(original.up_function_features, unmarshaled.up_function_features);
        assert_eq!(original.cp_function_features, unmarshaled.cp_function_features);
    }

    #[test]
    fn test_association_update_response_with_optional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );
        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03, 0x04]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x01, 0x02, 0x03, 0x04]);

        let original = AssociationUpdateResponse::new(
            456,
            node_id_ie,
            cause_ie,
            Some(up_features_ie),
            Some(cp_features_ie),
        );
        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_association_update_response_missing_required_ie() {
        // Test missing NodeId IE
        let incomplete_data = [
            0x21, 0x08, 0x00, 0x04, // Header (type=8, length=4, seq=0)
            0x00, 0x00, 0x00, 0x00, // No IEs following
        ];
        let result = AssociationUpdateResponse::unmarshal(&incomplete_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let message = AssociationUpdateResponse::new(123, node_id_ie, cause_ie, None, None);

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::Cause).is_some());
        assert!(message.find_ie(IeType::UpFunctionFeatures).is_none());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }
}