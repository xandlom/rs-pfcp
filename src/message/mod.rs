//! PFCP messages and their components.

pub mod association_release_request;
pub mod association_release_response;
pub mod association_setup_request;
pub mod association_setup_response;
pub mod association_update_request;
pub mod association_update_response;
pub mod display;
pub mod header;
pub mod heartbeat_request;
pub mod heartbeat_response;
pub mod ie_iter;
pub mod node_report_request;
pub mod node_report_response;
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
pub mod session_set_deletion_request;
pub mod session_set_deletion_response;
pub mod session_set_modification_request;
pub mod session_set_modification_response;
pub mod version_not_supported_response;

use crate::ie::Ie;
use std::io;

// Re-export IE iterator types for public API
pub use ie_iter::IeIter;

// Re-export message types for public API
pub use crate::message::association_release_request::AssociationReleaseRequest;
pub use crate::message::association_release_response::AssociationReleaseResponse;
pub use crate::message::association_setup_request::AssociationSetupRequest;
pub use crate::message::association_setup_response::AssociationSetupResponse;
pub use crate::message::association_update_request::AssociationUpdateRequest;
pub use crate::message::association_update_response::AssociationUpdateResponse;
pub use crate::message::heartbeat_request::HeartbeatRequest;
pub use crate::message::heartbeat_response::HeartbeatResponse;
pub use crate::message::node_report_request::NodeReportRequest;
pub use crate::message::node_report_response::NodeReportResponse;
pub use crate::message::pfd_management_request::PfdManagementRequest;
pub use crate::message::pfd_management_response::PfdManagementResponse;
pub use crate::message::session_deletion_request::SessionDeletionRequest;
pub use crate::message::session_deletion_response::SessionDeletionResponse;
pub use crate::message::session_establishment_request::SessionEstablishmentRequest;
pub use crate::message::session_establishment_response::SessionEstablishmentResponse;
pub use crate::message::session_modification_request::SessionModificationRequest;
pub use crate::message::session_modification_response::SessionModificationResponse;
pub use crate::message::session_report_request::SessionReportRequest;
pub use crate::message::session_report_response::SessionReportResponse;
pub use crate::message::session_set_deletion_request::SessionSetDeletionRequest;
pub use crate::message::session_set_deletion_response::SessionSetDeletionResponse;
pub use crate::message::session_set_modification_request::SessionSetModificationRequest;
pub use crate::message::session_set_modification_response::SessionSetModificationResponse;
pub use crate::message::version_not_supported_response::VersionNotSupportedResponse;

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
    SessionSetModificationRequest = 16,
    SessionSetModificationResponse = 17,
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
            16 => MsgType::SessionSetModificationRequest,
            17 => MsgType::SessionSetModificationResponse,
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
    /// Marshals the message into a new `Vec<u8>`.
    ///
    /// This is the standard marshaling method that allocates a new vector.
    /// For better performance in hot paths, consider using `marshal_into`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::{Message, heartbeat_request::HeartbeatRequest};
    /// use rs_pfcp::ie::{Ie, IeType, recovery_time_stamp::RecoveryTimeStamp};
    /// use std::time::SystemTime;
    ///
    /// let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    /// let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
    /// let request = HeartbeatRequest::new(123, ts_ie, None, vec![]);
    /// let bytes = request.marshal();
    /// ```
    fn marshal(&self) -> Vec<u8>;

    /// Marshals the message into an existing buffer.
    ///
    /// This method appends the marshaled message to the provided buffer,
    /// allowing for buffer reuse and avoiding allocations in hot paths.
    ///
    /// # Performance Note
    ///
    /// The default implementation calls `marshal()` and extends the buffer.
    /// Message types should override this for optimal performance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::{Message, heartbeat_request::HeartbeatRequest};
    /// use rs_pfcp::ie::{Ie, IeType, recovery_time_stamp::RecoveryTimeStamp};
    /// use std::time::SystemTime;
    ///
    /// let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    /// let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
    /// let request = HeartbeatRequest::new(123, ts_ie, None, vec![]);
    ///
    /// // Reuse buffer for multiple messages
    /// let mut buf = Vec::new();
    /// request.marshal_into(&mut buf);
    /// // Process buf...
    /// buf.clear();
    /// request.marshal_into(&mut buf);
    /// ```
    fn marshal_into(&self, buf: &mut Vec<u8>) {
        // Default implementation: use existing marshal() method
        buf.extend_from_slice(&self.marshal());
    }

    /// Returns the size of the marshaled message in bytes.
    ///
    /// This can be used to pre-allocate buffers of the correct size,
    /// avoiding reallocations during marshaling.
    ///
    /// # Performance Note
    ///
    /// The default implementation calls `marshal()` which allocates.
    /// Message types should override this for optimal performance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::{Message, heartbeat_request::HeartbeatRequest};
    /// use rs_pfcp::ie::{Ie, IeType, recovery_time_stamp::RecoveryTimeStamp};
    /// use std::time::SystemTime;
    ///
    /// let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    /// let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
    /// let request = HeartbeatRequest::new(123, ts_ie, None, vec![]);
    ///
    /// // Pre-allocate exact size needed
    /// let size = request.marshaled_size();
    /// let mut buf = Vec::with_capacity(size);
    /// request.marshal_into(&mut buf);
    /// assert_eq!(buf.len(), size);
    /// ```
    fn marshaled_size(&self) -> usize {
        // Default implementation: marshal to get size (inefficient but works)
        self.marshal().len()
    }

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

    /// Get all IEs of a specific type as an iterator.
    ///
    /// This is the recommended way to access IEs in a message. It provides a unified
    /// interface that works consistently whether the IE appears 0, 1, or many times.
    ///
    /// # Benefits
    ///
    /// - **Consistent API**: Same method works for all IE patterns (single, optional, multiple)
    /// - **Prevents bugs**: Iterator makes it clear you're getting all IEs, not just the first
    /// - **Ergonomic**: Use standard iterator methods (`.count()`, `.collect()`, `.map()`, etc.)
    /// - **Zero-cost**: Optimizes to direct access with no runtime overhead
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_pfcp::message::{Message, SessionEstablishmentRequest};
    /// use rs_pfcp::ie::{Ie, IeType, node_id::NodeId};
    /// use std::net::Ipv4Addr;
    ///
    /// # fn example(msg: &SessionEstablishmentRequest) {
    /// // Single mandatory IE
    /// if let Some(node_id_ie) = msg.ies(IeType::NodeId).next() {
    ///     println!("Found Node ID");
    /// }
    ///
    /// // Optional IE
    /// match msg.ies(IeType::PdnType).next() {
    ///     Some(pdn) => println!("PDN Type present"),
    ///     None => println!("No PDN Type"),
    /// }
    ///
    /// // Multiple IEs - iterate all
    /// for pdr in msg.ies(IeType::CreatePdr) {
    ///     println!("Processing PDR: {} bytes", pdr.len());
    /// }
    ///
    /// // Count IEs
    /// let pdr_count = msg.ies(IeType::CreatePdr).count();
    /// println!("Found {} PDRs", pdr_count);
    ///
    /// // Collect into vector
    /// let all_fars: Vec<_> = msg.ies(IeType::CreateFar).collect();
    /// # }
    /// ```
    fn ies(&self, ie_type: crate::ie::IeType) -> IeIter<'_>;

    /// Get the first IE of a specific type.
    ///
    /// **Deprecated:** This method only returns the first IE, which can lead to bugs
    /// when an IE appears multiple times. Use `ies(ie_type).next()` instead.
    ///
    /// # Migration
    ///
    /// ```rust
    /// # use rs_pfcp::message::{Message, SessionEstablishmentRequest};
    /// # use rs_pfcp::ie::IeType;
    /// # fn example(msg: &SessionEstablishmentRequest) {
    /// // Old way (deprecated)
    /// # #[allow(deprecated)]
    /// let node_id = msg.find_ie(IeType::NodeId);
    ///
    /// // New way (recommended)
    /// let node_id = msg.ies(IeType::NodeId).next();
    /// # }
    /// ```
    #[deprecated(
        since = "0.2.2",
        note = "Use `ies(ie_type).next()` instead. This method only returns the first IE, which can cause bugs with multiple IEs."
    )]
    fn find_ie(&self, ie_type: crate::ie::IeType) -> Option<&Ie> {
        self.ies(ie_type).next()
    }

    /// Get all IEs of a specific type as a vector.
    ///
    /// **Deprecated:** This method allocates a Vec. Use `ies(ie_type)` which returns
    /// an iterator for better ergonomics and performance.
    ///
    /// # Migration
    ///
    /// ```rust
    /// # use rs_pfcp::message::{Message, SessionEstablishmentRequest};
    /// # use rs_pfcp::ie::IeType;
    /// # fn example(msg: &SessionEstablishmentRequest) {
    /// // Old way (deprecated)
    /// # #[allow(deprecated)]
    /// let pdrs = msg.find_all_ies(IeType::CreatePdr);
    /// for pdr in pdrs {
    ///     // Process
    /// }
    ///
    /// // New way (recommended)
    /// for pdr in msg.ies(IeType::CreatePdr) {
    ///     // Process - no allocation!
    /// }
    ///
    /// // If you really need a Vec
    /// let pdrs: Vec<_> = msg.ies(IeType::CreatePdr).collect();
    /// # }
    /// ```
    #[deprecated(
        since = "0.2.2",
        note = "Use `ies(ie_type)` instead, which returns an iterator. Call `.collect()` if you need a Vec."
    )]
    fn find_all_ies(&self, ie_type: crate::ie::IeType) -> Vec<&Ie> {
        self.ies(ie_type).collect()
    }

    /// Get all IEs from the message.
    ///
    /// Returns a vector containing references to all Information Elements
    /// present in the message, regardless of type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::{Message, heartbeat_request::HeartbeatRequest};
    /// use rs_pfcp::ie::{Ie, IeType, recovery_time_stamp::RecoveryTimeStamp};
    /// use std::time::SystemTime;
    ///
    /// let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    /// let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
    /// let request = HeartbeatRequest::new(123, ts_ie, None, vec![]);
    /// let all_ies = request.all_ies();
    /// println!("Message contains {} IEs", all_ies.len());
    /// ```
    fn all_ies(&self) -> Vec<&Ie>;
}

// A generic message for unknown message types.
pub struct Generic {
    header: header::Header,
    ies: Vec<Ie>,
}

impl Message for Generic {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
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

    fn ies(&self, ie_type: crate::ie::IeType) -> IeIter<'_> {
        IeIter::generic(&self.ies, ie_type)
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: crate::ie::IeType) -> Option<&Ie> {
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }

    fn all_ies(&self) -> Vec<&Ie> {
        self.ies.iter().collect()
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
        MsgType::AssociationUpdateResponse => {
            Ok(Box::new(AssociationUpdateResponse::unmarshal(data)?))
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
        MsgType::VersionNotSupportedResponse => {
            Ok(Box::new(VersionNotSupportedResponse::unmarshal(data)?))
        }
        MsgType::NodeReportRequest => Ok(Box::new(NodeReportRequest::unmarshal(data)?)),
        MsgType::NodeReportResponse => Ok(Box::new(NodeReportResponse::unmarshal(data)?)),
        MsgType::SessionSetDeletionRequest => {
            Ok(Box::new(SessionSetDeletionRequest::unmarshal(data)?))
        }
        MsgType::SessionSetDeletionResponse => {
            Ok(Box::new(SessionSetDeletionResponse::unmarshal(data)?))
        }
        MsgType::SessionSetModificationRequest => {
            Ok(Box::new(SessionSetModificationRequest::unmarshal(data)?))
        }
        MsgType::SessionSetModificationResponse => {
            Ok(Box::new(SessionSetModificationResponse::unmarshal(data)?))
        }
        _ => Ok(Box::new(Generic::unmarshal(data)?)),
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::ie::{Ie, IeType};
    use crate::message::header::Header;

    // ========================================================================
    // MsgType Conversion Tests
    // ========================================================================

    #[test]
    fn test_msg_type_from_u8_heartbeat() {
        assert_eq!(MsgType::from(1), MsgType::HeartbeatRequest);
        assert_eq!(MsgType::from(2), MsgType::HeartbeatResponse);
    }

    #[test]
    fn test_msg_type_from_u8_pfd_management() {
        assert_eq!(MsgType::from(3), MsgType::PfdManagementRequest);
        assert_eq!(MsgType::from(4), MsgType::PfdManagementResponse);
    }

    #[test]
    fn test_msg_type_from_u8_association() {
        assert_eq!(MsgType::from(5), MsgType::AssociationSetupRequest);
        assert_eq!(MsgType::from(6), MsgType::AssociationSetupResponse);
        assert_eq!(MsgType::from(7), MsgType::AssociationUpdateRequest);
        assert_eq!(MsgType::from(8), MsgType::AssociationUpdateResponse);
        assert_eq!(MsgType::from(9), MsgType::AssociationReleaseRequest);
        assert_eq!(MsgType::from(10), MsgType::AssociationReleaseResponse);
    }

    #[test]
    fn test_msg_type_from_u8_session() {
        assert_eq!(MsgType::from(50), MsgType::SessionEstablishmentRequest);
        assert_eq!(MsgType::from(51), MsgType::SessionEstablishmentResponse);
        assert_eq!(MsgType::from(52), MsgType::SessionModificationRequest);
        assert_eq!(MsgType::from(53), MsgType::SessionModificationResponse);
        assert_eq!(MsgType::from(54), MsgType::SessionDeletionRequest);
        assert_eq!(MsgType::from(55), MsgType::SessionDeletionResponse);
        assert_eq!(MsgType::from(56), MsgType::SessionReportRequest);
        assert_eq!(MsgType::from(57), MsgType::SessionReportResponse);
    }

    #[test]
    fn test_msg_type_from_u8_node_report() {
        assert_eq!(MsgType::from(12), MsgType::NodeReportRequest);
        assert_eq!(MsgType::from(13), MsgType::NodeReportResponse);
    }

    #[test]
    fn test_msg_type_from_u8_session_set() {
        assert_eq!(MsgType::from(14), MsgType::SessionSetDeletionRequest);
        assert_eq!(MsgType::from(15), MsgType::SessionSetDeletionResponse);
        assert_eq!(MsgType::from(16), MsgType::SessionSetModificationRequest);
        assert_eq!(MsgType::from(17), MsgType::SessionSetModificationResponse);
    }

    #[test]
    fn test_msg_type_from_u8_version_not_supported() {
        assert_eq!(MsgType::from(11), MsgType::VersionNotSupportedResponse);
    }

    #[test]
    fn test_msg_type_from_u8_unknown() {
        assert_eq!(MsgType::from(0), MsgType::Unknown);
        assert_eq!(MsgType::from(18), MsgType::Unknown);
        assert_eq!(MsgType::from(255), MsgType::Unknown);
    }

    #[test]
    fn test_msg_type_to_u8_round_trip() {
        let test_types = vec![
            MsgType::HeartbeatRequest,
            MsgType::HeartbeatResponse,
            MsgType::AssociationSetupRequest,
            MsgType::AssociationSetupResponse,
            MsgType::SessionEstablishmentRequest,
            MsgType::SessionEstablishmentResponse,
            MsgType::SessionModificationRequest,
            MsgType::SessionModificationResponse,
            MsgType::SessionDeletionRequest,
            MsgType::SessionDeletionResponse,
            MsgType::SessionReportRequest,
            MsgType::SessionReportResponse,
        ];

        for msg_type in test_types {
            let as_u8 = msg_type as u8;
            let back = MsgType::from(as_u8);
            assert_eq!(back, msg_type, "Round-trip failed for {:?}", msg_type);
        }
    }

    // ========================================================================
    // Generic Message Tests
    // ========================================================================

    #[test]
    fn test_generic_message_marshal_unmarshal() {
        // Create a minimal PFCP message with Generic
        let header = Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            message_type: MsgType::Unknown,
            length: 8, // Will be recalculated
            seid: 0,
            sequence_number: 12345,
            message_priority: 0,
            has_seid: false,
        };

        let ies = vec![
            Ie::new(IeType::Cause, vec![0x01]), // Request accepted
        ];

        let msg = Generic {
            header: header.clone(),
            ies: ies.clone(),
        };

        let marshaled = msg.marshal();
        let unmarshaled = Generic::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.msg_type(), msg.msg_type());
        assert_eq!(unmarshaled.sequence(), msg.sequence());
        assert_eq!(unmarshaled.seid(), msg.seid());
        assert_eq!(unmarshaled.ies.len(), ies.len());
    }

    #[test]
    fn test_generic_message_with_seid() {
        let header = Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            message_type: MsgType::Unknown,
            length: 16,
            seid: 0x1234567890ABCDEF,
            sequence_number: 54321,
            message_priority: 0,
            has_seid: true,
        };

        let msg = Generic {
            header,
            ies: vec![],
        };

        assert_eq!(msg.seid(), Some(0x1234567890ABCDEF));
    }

    #[test]
    fn test_generic_message_without_seid() {
        let header = Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            message_type: MsgType::HeartbeatRequest,
            length: 8,
            seid: 0,
            sequence_number: 111,
            message_priority: 0,
            has_seid: false,
        };

        let msg = Generic {
            header,
            ies: vec![],
        };

        assert_eq!(msg.seid(), None);
    }

    #[test]
    fn test_generic_message_set_sequence() {
        let mut msg = Generic {
            header: Header {
                version: 1,
                has_fo: false,
                has_mp: false,
                message_type: MsgType::Unknown,
                length: 8,
                seid: 0,
                sequence_number: 100,
                message_priority: 0,
                has_seid: false,
            },
            ies: vec![],
        };

        assert_eq!(msg.sequence(), 100);
        msg.set_sequence(200);
        assert_eq!(msg.sequence(), 200);
    }

    #[test]
    fn test_generic_message_find_ie() {
        let ies = vec![
            Ie::new(IeType::Cause, vec![0x01]),
            Ie::new(IeType::NodeId, vec![0x00, 0x01, 0x02, 0x03, 0x04]),
            Ie::new(IeType::RecoveryTimeStamp, vec![0x00, 0x00, 0x00, 0x01]),
        ];

        let msg = Generic {
            header: Header {
                version: 1,
                has_fo: false,
                has_mp: false,
                message_type: MsgType::Unknown,
                length: 8,
                seid: 0,
                sequence_number: 1,
                message_priority: 0,
                has_seid: false,
            },
            ies,
        };

        // Find existing IE
        let cause = msg.find_ie(IeType::Cause);
        assert!(cause.is_some());
        assert_eq!(cause.unwrap().ie_type, IeType::Cause);

        // Find non-existent IE
        let far_id = msg.find_ie(IeType::FarId);
        assert!(far_id.is_none());
    }

    #[test]
    fn test_generic_message_find_all_ies() {
        let ies = vec![
            Ie::new(IeType::Cause, vec![0x01]),
            Ie::new(IeType::CreatePdr, vec![0x01, 0x02]),
            Ie::new(IeType::CreatePdr, vec![0x03, 0x04]),
            Ie::new(IeType::NodeId, vec![0x00, 0x01, 0x02, 0x03, 0x04]),
        ];

        let msg = Generic {
            header: Header {
                version: 1,
                has_fo: false,
                has_mp: false,
                message_type: MsgType::Unknown,
                length: 8,
                seid: 0,
                sequence_number: 1,
                message_priority: 0,
                has_seid: false,
            },
            ies,
        };

        // Default implementation returns single IE or empty vec
        let cause_ies = msg.find_all_ies(IeType::Cause);
        assert_eq!(cause_ies.len(), 1);

        // Non-existent IE
        let far_ies = msg.find_all_ies(IeType::FarId);
        assert_eq!(far_ies.len(), 0);
    }

    #[test]
    fn test_generic_message_msg_name() {
        let msg = Generic {
            header: Header {
                version: 1,
                has_fo: false,
                has_mp: false,
                message_type: MsgType::HeartbeatRequest,
                length: 8,
                seid: 0,
                sequence_number: 1,
                message_priority: 0,
                has_seid: false,
            },
            ies: vec![],
        };

        assert_eq!(msg.msg_name(), "HeartbeatRequest");
    }

    #[test]
    fn test_generic_message_version() {
        let msg = Generic {
            header: Header {
                version: 1,
                has_fo: false,
                has_mp: false,
                message_type: MsgType::Unknown,
                length: 8,
                seid: 0,
                sequence_number: 1,
                message_priority: 0,
                has_seid: false,
            },
            ies: vec![],
        };

        assert_eq!(msg.version(), 1);
    }

    #[test]
    fn test_generic_message_unmarshal_error_invalid_header() {
        let short_data = vec![0x20]; // Too short for header
        let result = Generic::unmarshal(&short_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_message_unmarshal_multiple_ies() {
        // Create Generic with multiple IEs
        let header = Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            message_type: MsgType::Unknown,
            length: 8,
            seid: 0,
            sequence_number: 999,
            message_priority: 0,
            has_seid: false,
        };

        let ies = vec![
            Ie::new(IeType::Cause, vec![0x01]),
            Ie::new(IeType::NodeId, vec![0x00, 0xC0, 0xA8, 0x01, 0x01]),
            Ie::new(IeType::RecoveryTimeStamp, vec![0x12, 0x34, 0x56, 0x78]),
        ];

        let msg = Generic { header, ies };
        let marshaled = msg.marshal();

        let unmarshaled = Generic::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.ies.len(), 3);
        assert_eq!(unmarshaled.ies[0].ie_type, IeType::Cause);
        assert_eq!(unmarshaled.ies[1].ie_type, IeType::NodeId);
        assert_eq!(unmarshaled.ies[2].ie_type, IeType::RecoveryTimeStamp);
    }

    // ========================================================================
    // Parse Function Tests
    // ========================================================================

    #[test]
    fn test_parse_heartbeat_request() {
        // Create a valid heartbeat request message
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        use crate::message::heartbeat_request::HeartbeatRequestBuilder;
        use std::time::SystemTime;

        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let msg = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::HeartbeatRequest);
        assert_eq!(parsed.sequence(), 12345);
    }

    #[test]
    fn test_parse_heartbeat_response() {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        use crate::message::heartbeat_response::HeartbeatResponseBuilder;
        use std::time::SystemTime;

        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let msg = HeartbeatResponseBuilder::new(54321)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::HeartbeatResponse);
        assert_eq!(parsed.sequence(), 54321);
    }

    #[test]
    fn test_parse_association_setup_request() {
        use crate::message::association_setup_request::AssociationSetupRequestBuilder;
        use std::net::Ipv4Addr;
        use std::time::SystemTime;

        let msg = AssociationSetupRequestBuilder::new(11111)
            .node_id(Ipv4Addr::new(192, 168, 1, 1))
            .recovery_time_stamp(SystemTime::now())
            .build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::AssociationSetupRequest);
        assert_eq!(parsed.sequence(), 11111);
    }

    #[test]
    fn test_parse_session_establishment_request() {
        use crate::message::session_establishment_request::SessionEstablishmentRequestBuilder;
        use std::net::{IpAddr, Ipv4Addr};

        // Create minimal PDR and FAR IEs
        let pdr_ie = Ie::new(
            IeType::CreatePdr,
            vec![
                0, 56, 0, 2, // PDR ID IE
                0, 1, // PDR ID value = 1
            ],
        );

        let far_ie = Ie::new(
            IeType::CreateFar,
            vec![
                0, 108, 0, 4, // FAR ID IE
                0, 0, 0, 1, // FAR ID value = 1
            ],
        );

        let msg = SessionEstablishmentRequestBuilder::new(0x1234567890ABCDEF, 99999)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .fseid(0x1234567890ABCDEF, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))
            .create_pdrs(vec![pdr_ie])
            .create_fars(vec![far_ie])
            .build()
            .unwrap();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionEstablishmentRequest);
        assert_eq!(parsed.sequence(), 99999);
        assert_eq!(parsed.seid(), Some(0x1234567890ABCDEF));
    }

    #[test]
    fn test_parse_session_establishment_response() {
        use crate::message::session_establishment_response::SessionEstablishmentResponseBuilder;
        use std::net::{IpAddr, Ipv4Addr};

        let msg = SessionEstablishmentResponseBuilder::accepted(0xABCDEF, 77777)
            .fseid(0xABCDEF, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)))
            .marshal()
            .unwrap();

        let parsed = parse(&msg).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionEstablishmentResponse);
        assert_eq!(parsed.sequence(), 77777);
        assert_eq!(parsed.seid(), Some(0xABCDEF));
    }

    #[test]
    fn test_parse_session_modification_request() {
        use crate::message::session_modification_request::SessionModificationRequestBuilder;

        let msg = SessionModificationRequestBuilder::new(0x123456, 88888).build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionModificationRequest);
        assert_eq!(parsed.sequence(), 88888);
        assert_eq!(parsed.seid(), Some(0x123456));
    }

    #[test]
    fn test_parse_session_deletion_request() {
        use crate::message::session_deletion_request::SessionDeletionRequestBuilder;

        // Minimal deletion request with SEID in header (per 3GPP TS 29.244 Section 7.5.6)
        let msg = SessionDeletionRequestBuilder::new(0x999999, 66666).build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionDeletionRequest);
        assert_eq!(parsed.sequence(), 66666);
        assert_eq!(parsed.seid(), Some(0x999999));
    }

    #[test]
    fn test_parse_session_report_request() {
        use crate::message::session_report_request::SessionReportRequestBuilder;

        let msg = SessionReportRequestBuilder::new(0xFEDCBA, 55555).build();

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionReportRequest);
        assert_eq!(parsed.sequence(), 55555);
        assert_eq!(parsed.seid(), Some(0xFEDCBA));
    }

    #[test]
    fn test_parse_unknown_message_type() {
        // Create a message with unknown type (using Generic)
        let header = Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            message_type: MsgType::Unknown,
            length: 8,
            seid: 0,
            sequence_number: 33333,
            message_priority: 0,
            has_seid: false,
        };

        let msg = Generic {
            header,
            ies: vec![],
        };

        let marshaled = msg.marshal();
        let parsed = parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::Unknown);
        assert_eq!(parsed.sequence(), 33333);
    }

    #[test]
    fn test_parse_error_short_buffer() {
        let short_data = vec![0x20, 0x01]; // Too short for valid message
        let result = parse(&short_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_all_message_types() {
        // Test that parse function handles all known message types
        let message_types = vec![
            (MsgType::HeartbeatRequest, 1u8),
            (MsgType::HeartbeatResponse, 2u8),
            (MsgType::PfdManagementRequest, 3u8),
            (MsgType::PfdManagementResponse, 4u8),
            (MsgType::AssociationSetupRequest, 5u8),
            (MsgType::AssociationSetupResponse, 6u8),
            (MsgType::AssociationUpdateRequest, 7u8),
            (MsgType::AssociationUpdateResponse, 8u8),
            (MsgType::AssociationReleaseRequest, 9u8),
            (MsgType::AssociationReleaseResponse, 10u8),
            (MsgType::VersionNotSupportedResponse, 11u8),
            (MsgType::NodeReportRequest, 12u8),
            (MsgType::NodeReportResponse, 13u8),
            (MsgType::SessionSetDeletionRequest, 14u8),
            (MsgType::SessionSetDeletionResponse, 15u8),
            (MsgType::SessionSetModificationRequest, 16u8),
            (MsgType::SessionSetModificationResponse, 17u8),
            (MsgType::SessionEstablishmentRequest, 50u8),
            (MsgType::SessionEstablishmentResponse, 51u8),
            (MsgType::SessionModificationRequest, 52u8),
            (MsgType::SessionModificationResponse, 53u8),
            (MsgType::SessionDeletionRequest, 54u8),
            (MsgType::SessionDeletionResponse, 55u8),
            (MsgType::SessionReportRequest, 56u8),
            (MsgType::SessionReportResponse, 57u8),
        ];

        for (expected_type, type_value) in message_types {
            assert_eq!(
                MsgType::from(type_value),
                expected_type,
                "MsgType conversion failed for {:?}",
                expected_type
            );
        }
    }

    #[test]
    fn test_parse_round_trip_preserves_ies() {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        use crate::message::heartbeat_request::HeartbeatRequestBuilder;
        use std::time::SystemTime;

        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let original = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie.clone())
            .build();

        let marshaled = original.marshal();
        let parsed = parse(&marshaled).unwrap();

        // Verify IE is preserved
        let found_ie = parsed.find_ie(IeType::RecoveryTimeStamp);
        assert!(found_ie.is_some());
        assert_eq!(found_ie.unwrap().ie_type, IeType::RecoveryTimeStamp);
    }
}
