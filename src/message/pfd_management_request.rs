//! PFD Management Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementRequest {
    pub header: Header,
    pub application_ids_pfds: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl PfdManagementRequest {
    /// Creates a new PFD Management Request message.
    pub fn new(seq: u32, ies: Vec<Ie>) -> Self {
        let mut application_ids_pfds = Vec::new();
        let mut other_ies = Vec::new();
        let mut payload_len = 0;

        for ie in ies {
            payload_len += ie.len();
            if ie.ie_type == IeType::ApplicationIdsPfds {
                application_ids_pfds.push(ie);
            } else {
                other_ies.push(ie);
            }
        }

        let mut header = Header::new(MsgType::PfdManagementRequest, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementRequest {
            header,
            application_ids_pfds,
            ies: other_ies,
        }
    }
}

impl Message for PfdManagementRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        for ie in &self.application_ids_pfds {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut application_ids_pfds = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::ApplicationIdsPfds => application_ids_pfds.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementRequest {
            header,
            application_ids_pfds,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementRequest
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
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

/// Builder for PfdManagementRequest message.
#[derive(Debug, Default)]
pub struct PfdManagementRequestBuilder {
    sequence: u32,
    application_ids_pfds: Vec<Ie>,
    ies: Vec<Ie>,
}

impl PfdManagementRequestBuilder {
    /// Creates a new PfdManagementRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            application_ids_pfds: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Adds an Application IDs PFDs IE.
    pub fn application_ids_pfds(mut self, application_ids_pfds: Ie) -> Self {
        self.application_ids_pfds.push(application_ids_pfds);
        self
    }

    /// Adds multiple Application IDs PFDs IEs.
    pub fn application_ids_pfds_vec(mut self, mut application_ids_pfds: Vec<Ie>) -> Self {
        self.application_ids_pfds.append(&mut application_ids_pfds);
        self
    }

    /// Adds an additional IE (non-ApplicationIdsPfds).
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple additional IEs (non-ApplicationIdsPfds).
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the PfdManagementRequest message.
    pub fn build(self) -> PfdManagementRequest {
        // Combine all IEs and let the new() method separate them correctly
        let mut all_ies = self.application_ids_pfds;
        all_ies.extend(self.ies);

        PfdManagementRequest::new(self.sequence, all_ies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfd_management_request_builder_minimal() {
        let request = PfdManagementRequestBuilder::new(12345).build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.msg_type(), MsgType::PfdManagementRequest);
        assert!(request.application_ids_pfds.is_empty());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_application_ids_pfds() {
        let app_id_pfd_ie = Ie::new(IeType::ApplicationIdsPfds, vec![0x01, 0x02, 0x03]);

        let request = PfdManagementRequestBuilder::new(12345)
            .application_ids_pfds(app_id_pfd_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.application_ids_pfds.len(), 1);
        assert_eq!(request.application_ids_pfds[0], app_id_pfd_ie);
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_multiple_application_ids_pfds() {
        let app_ie1 = Ie::new(IeType::ApplicationIdsPfds, vec![0x01, 0x02]);
        let app_ie2 = Ie::new(IeType::ApplicationIdsPfds, vec![0x03, 0x04]);
        let app_ie3 = Ie::new(IeType::ApplicationIdsPfds, vec![0x05, 0x06]);

        let request = PfdManagementRequestBuilder::new(98765)
            .application_ids_pfds(app_ie1.clone())
            .application_ids_pfds_vec(vec![app_ie2.clone(), app_ie3.clone()])
            .build();

        assert_eq!(request.sequence(), 98765);
        assert_eq!(request.application_ids_pfds.len(), 3);
        assert_eq!(request.application_ids_pfds[0], app_ie1);
        assert_eq!(request.application_ids_pfds[1], app_ie2);
        assert_eq!(request.application_ids_pfds[2], app_ie3);
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_other_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let request = PfdManagementRequestBuilder::new(55555)
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.sequence(), 55555);
        assert!(request.application_ids_pfds.is_empty());
        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_pfd_management_request_builder_full() {
        let app_ie = Ie::new(IeType::ApplicationIdsPfds, vec![0x01, 0x02, 0x03, 0x04]);
        let other_ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let other_ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);

        let request = PfdManagementRequestBuilder::new(77777)
            .application_ids_pfds(app_ie.clone())
            .ie(other_ie1.clone())
            .ie(other_ie2.clone())
            .build();

        assert_eq!(request.sequence(), 77777);
        assert_eq!(request.application_ids_pfds.len(), 1);
        assert_eq!(request.application_ids_pfds[0], app_ie);
        assert_eq!(request.ies.len(), 2);
        assert_eq!(request.ies[0], other_ie1);
        assert_eq!(request.ies[1], other_ie2);
    }

    #[test]
    fn test_pfd_management_request_builder_roundtrip() {
        let app_ie1 = Ie::new(IeType::ApplicationIdsPfds, vec![0x11, 0x22]);
        let app_ie2 = Ie::new(IeType::ApplicationIdsPfds, vec![0x33, 0x44]);
        let other_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let original = PfdManagementRequestBuilder::new(99999)
            .application_ids_pfds(app_ie1)
            .application_ids_pfds(app_ie2)
            .ie(other_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = PfdManagementRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_pfd_management_request_builder_ie_separation() {
        // Test that the builder correctly separates ApplicationIdsPfds from other IEs
        // even when mixed together in the build process
        let app_ie = Ie::new(IeType::ApplicationIdsPfds, vec![0x01, 0x02]);
        let other_ie = Ie::new(IeType::Unknown, vec![0x03, 0x04]);

        let request = PfdManagementRequestBuilder::new(11111)
            .ie(other_ie.clone()) // Add other IE first
            .application_ids_pfds(app_ie.clone()) // Then add app IE
            .build();

        // Verify correct separation occurred
        assert_eq!(request.application_ids_pfds.len(), 1);
        assert_eq!(request.application_ids_pfds[0], app_ie);
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], other_ie);
    }
}
