//! PFCP message header.

use crate::message::MsgType;
use crate::error::PfcpError;
use std::io;

/// Represents a PFCP message header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub has_fo: bool, // Follow-on
    pub has_mp: bool, // Message Priority
    pub has_seid: bool,
    pub message_type: MsgType,
    pub length: u16,
    pub seid: u64,
    pub sequence_number: u32,
    pub message_priority: u8,
}

impl Header {
    /// Creates a new Header.
    pub fn new(message_type: MsgType, has_seid: bool, seid: u64, sequence_number: u32) -> Self {
        Header {
            version: 1,
            has_fo: false,
            has_mp: false,
            has_seid,
            message_type,
            length: 0, // Will be set later
            seid,
            sequence_number,
            message_priority: 0,
        }
    }

    /// Returns the length of the header in bytes.
    pub fn len(&self) -> u16 {
        let mut length = 8;
        if self.has_seid {
            length += 8;
        }
        length
    }

    /// Reports whether a Header is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Serializes the Header into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = vec![0; self.len() as usize];
        self.marshal_to(&mut data);
        data
    }

    /// Serializes the Header into an existing buffer.
    ///
    /// This method appends the marshaled header to the provided buffer,
    /// allowing for buffer reuse and avoiding allocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::header::Header;
    /// use rs_pfcp::message::MsgType;
    ///
    /// let header = Header::new(MsgType::HeartbeatRequest, false, 0, 123);
    ///
    /// // Reuse buffer
    /// let mut buf = Vec::new();
    /// header.marshal_into(&mut buf);
    /// assert_eq!(buf.len(), header.len() as usize);
    /// ```
    pub fn marshal_into(&self, buf: &mut Vec<u8>) {
        let start = buf.len();
        buf.resize(start + self.len() as usize, 0);
        self.marshal_to(&mut buf[start..]);
    }

    /// Serializes the Header into a byte slice.
    pub fn marshal_to(&self, b: &mut [u8]) {
        let flags = (self.version << 5)
            | ((self.has_fo as u8) << 2)
            | ((self.has_mp as u8) << 1)
            | (self.has_seid as u8);
        b[0] = flags;
        b[1] = self.message_type as u8;

        b[2..4].copy_from_slice(&self.length.to_be_bytes());

        let mut offset = 4;
        if self.has_seid {
            b[offset..offset + 8].copy_from_slice(&self.seid.to_be_bytes());
            offset += 8;
        }

        let seq_bytes = self.sequence_number.to_be_bytes();
        b[offset..offset + 3].copy_from_slice(&seq_bytes[1..]);
        b[offset + 3] = self.message_priority;
    }

    /// Deserializes a byte slice into a Header.
    pub fn unmarshal(b: &[u8]) -> Result<Self, PfcpError> {
        if b.len() < 8 {
            return Err(PfcpError::InvalidHeader {
                reason: "Header too short".into(),
                position: Some(0),
            });
        }

        let flags = b[0];
        let version = flags >> 5;
        let has_fo = (flags & 0x04) >> 2 == 1;
        let has_mp = (flags & 0x02) >> 1 == 1;
        let has_seid = (flags & 0x01) == 1;

        let message_type = MsgType::from(b[1]);
        let length = u16::from_be_bytes([b[2], b[3]]);

        let mut offset = 4;
        let seid = if has_seid {
            if b.len() < offset + 8 {
                return Err(PfcpError::InvalidHeader {
                    reason: "Header with SEID too short".into(),
                    position: Some(offset),
                });
            }
            offset += 8;
            u64::from_be_bytes([
                b[offset - 8],
                b[offset - 7],
                b[offset - 6],
                b[offset - 5],
                b[offset - 4],
                b[offset - 3],
                b[offset - 2],
                b[offset - 1],
            ])
        } else {
            0
        };

        if b.len() < offset + 4 {
            return Err(PfcpError::InvalidHeader {
                reason: "Header sequence number part too short".into(),
                position: Some(offset),
            });
        }
        let sequence_number = u32::from_be_bytes([0, b[offset], b[offset + 1], b[offset + 2]]);
        let message_priority = b[offset + 3];

        Ok(Header {
            version,
            has_fo,
            has_mp,
            has_seid,
            message_type,
            length,
            seid,
            sequence_number,
            message_priority,
        })
    }
}
