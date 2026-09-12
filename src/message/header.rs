//! PFCP message header.

use crate::error::PfcpError;
use crate::message::MsgType;
use crate::types::{Seid, SequenceNumber};

/// PFCP protocol version.
///
/// Per 3GPP TS 29.244 Section 5.1, the version field is a 3-bit value in the
/// header's first octet. Only version 1 has ever been defined.
pub const PFCP_VERSION: u8 = 1;

/// Length in bytes of the base PFCP header (flags, message type, length,
/// sequence number, and spare/message-priority octet) when the optional
/// SEID field is absent. Per 3GPP TS 29.244 Figure 5.1-1.
const BASE_HEADER_LEN: u16 = 8;

/// Length in bytes of the fixed header prefix (flags octet, message type
/// octet, and 2-byte length field) that precedes the optional SEID field
/// and the sequence number.
const HEADER_PREFIX_LEN: usize = 4;

/// Length in bytes of the SEID field (Session Endpoint Identifier is a
/// 64-bit value), present only when the S flag is set.
const SEID_FIELD_LEN: u16 = 8;

/// Length in bytes of the on-wire Sequence Number field.
///
/// Per 3GPP TS 29.244 Section 5.1, the sequence number is a 24-bit value
/// even though it is stored in-memory as a `u32` (see [`SequenceNumber`]).
const SEQUENCE_NUMBER_FIELD_LEN: usize = 3;

/// Length in bytes of the message-priority / spare octet that follows the
/// sequence number.
const MESSAGE_PRIORITY_FIELD_LEN: usize = 1;

/// Bit position of the 3-bit Version field within the header's first octet.
const VERSION_SHIFT: u8 = 5;
/// Bit position of the FO (Follow-On) flag within the header's first octet.
const FO_FLAG_SHIFT: u8 = 2;
/// Bit position of the MP (Message Priority) flag within the header's first octet.
const MP_FLAG_SHIFT: u8 = 1;
/// Bit position of the S (SEID present) flag within the header's first octet.
const SEID_FLAG_SHIFT: u8 = 0;

/// Bitmask isolating the FO flag in the header's first octet.
const FO_FLAG_MASK: u8 = 1 << FO_FLAG_SHIFT;
/// Bitmask isolating the MP flag in the header's first octet.
const MP_FLAG_MASK: u8 = 1 << MP_FLAG_SHIFT;
/// Bitmask isolating the S flag in the header's first octet.
const SEID_FLAG_MASK: u8 = 1 << SEID_FLAG_SHIFT;

/// Represents a PFCP message header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub has_fo: bool, // Follow-on
    pub has_mp: bool, // Message Priority
    pub has_seid: bool,
    pub message_type: MsgType,
    pub length: u16,
    pub seid: Seid,
    pub sequence_number: SequenceNumber,
    pub message_priority: u8,
    pub(crate) raw_message_type: u8,
}

impl Header {
    /// Creates a new Header.
    pub fn new(
        message_type: MsgType,
        has_seid: bool,
        seid: impl Into<Seid>,
        sequence_number: impl Into<SequenceNumber>,
    ) -> Self {
        Header {
            version: PFCP_VERSION,
            has_fo: false,
            has_mp: false,
            has_seid,
            message_type,
            length: 0, // Will be set later
            seid: seid.into(),
            sequence_number: sequence_number.into(),
            message_priority: 0,
            raw_message_type: message_type as u8,
        }
    }

    /// Creates a header for a message type not known by this crate.
    ///
    /// The raw type is retained so that decoding and re-encoding a future
    /// message does not silently change its type on the wire.
    pub fn new_unknown(
        raw_message_type: u8,
        has_seid: bool,
        seid: impl Into<Seid>,
        sequence_number: impl Into<SequenceNumber>,
    ) -> Result<Self, PfcpError> {
        if MsgType::from(raw_message_type) != MsgType::Unknown {
            return Err(PfcpError::invalid_value(
                "PFCP message type",
                raw_message_type.to_string(),
                "must be a message type unknown to this crate",
            ));
        }
        let mut header = Self::new(MsgType::Unknown, has_seid, seid, sequence_number);
        header.raw_message_type = raw_message_type;
        Ok(header)
    }

    /// Returns the message type value as encoded on the wire.
    pub fn message_type_code(&self) -> u8 {
        if self.message_type == MsgType::Unknown {
            self.raw_message_type
        } else {
            self.message_type as u8
        }
    }

    /// Returns the length of the header in bytes.
    pub fn len(&self) -> u16 {
        let mut length = BASE_HEADER_LEN;
        if self.has_seid {
            length += SEID_FIELD_LEN;
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
        let flags = (self.version << VERSION_SHIFT)
            | ((self.has_fo as u8) << FO_FLAG_SHIFT)
            | ((self.has_mp as u8) << MP_FLAG_SHIFT)
            | ((self.has_seid as u8) << SEID_FLAG_SHIFT);
        b[0] = flags;
        b[1] = self.message_type_code();

        b[2..HEADER_PREFIX_LEN].copy_from_slice(&self.length.to_be_bytes());

        let mut offset = HEADER_PREFIX_LEN;
        if self.has_seid {
            let seid_end = offset + SEID_FIELD_LEN as usize;
            b[offset..seid_end].copy_from_slice(&self.seid.0.to_be_bytes());
            offset = seid_end;
        }

        let seq_bytes = self.sequence_number.0.to_be_bytes();
        let seq_end = offset + SEQUENCE_NUMBER_FIELD_LEN;
        b[offset..seq_end]
            .copy_from_slice(&seq_bytes[seq_bytes.len() - SEQUENCE_NUMBER_FIELD_LEN..]);
        b[seq_end] = self.message_priority;
    }

    /// Deserializes a byte slice into a Header.
    pub fn unmarshal(b: &[u8]) -> Result<Self, PfcpError> {
        if b.len() < BASE_HEADER_LEN as usize {
            return Err(PfcpError::MessageParseError {
                message_type: None,
                reason: format!(
                    "Header too short (expected at least {} bytes, got {})",
                    BASE_HEADER_LEN,
                    b.len()
                ),
            });
        }

        let flags = b[0];
        let version = flags >> VERSION_SHIFT;
        let has_fo = (flags & FO_FLAG_MASK) >> FO_FLAG_SHIFT == 1;
        let has_mp = (flags & MP_FLAG_MASK) >> MP_FLAG_SHIFT == 1;
        let has_seid = (flags & SEID_FLAG_MASK) >> SEID_FLAG_SHIFT == 1;

        let raw_message_type = b[1];
        let message_type = MsgType::from(raw_message_type);
        let length = u16::from_be_bytes([b[2], b[3]]);

        let mut offset = HEADER_PREFIX_LEN;
        let seid_field_len = SEID_FIELD_LEN as usize;
        let seid = if has_seid {
            if b.len() < offset + seid_field_len {
                return Err(PfcpError::MessageParseError {
                    message_type: Some(message_type),
                    reason: format!(
                        "Header with SEID flag set but too short (expected at least {} bytes, got {})",
                        offset + seid_field_len,
                        b.len()
                    ),
                });
            }
            offset += seid_field_len;
            u64::from_be_bytes(
                b[offset - seid_field_len..offset]
                    .try_into()
                    .expect("slice has exactly SEID_FIELD_LEN bytes"),
            )
        } else {
            0
        };

        let trailer_len = SEQUENCE_NUMBER_FIELD_LEN + MESSAGE_PRIORITY_FIELD_LEN;
        if b.len() < offset + trailer_len {
            return Err(PfcpError::MessageParseError {
                message_type: Some(message_type),
                reason: format!(
                    "Header sequence number part too short (expected at least {} bytes, got {})",
                    offset + trailer_len,
                    b.len()
                ),
            });
        }
        let sequence_number = SequenceNumber::new(u32::from_be_bytes([
            0,
            b[offset],
            b[offset + 1],
            b[offset + 2],
        ]));
        let message_priority = b[offset + SEQUENCE_NUMBER_FIELD_LEN];

        Ok(Header {
            version,
            has_fo,
            has_mp,
            has_seid,
            message_type,
            length,
            seid: Seid(seid),
            sequence_number,
            message_priority,
            raw_message_type,
        })
    }
}
