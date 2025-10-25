//! Linked URR ID IE implementation.
//!
//! Per 3GPP TS 29.244 Section 8.2.83, the Linked URR ID IE is used to
//! indicate one or more URRs to which the current URR is linked.

use crate::ie::{Ie, IeType};
use std::io;

/// Linked URR ID Information Element.
///
/// Used to link Usage Reporting Rules together. When a URR is linked to
/// another URR, usage reports from one may trigger actions on the other.
///
/// # Structure (per 3GPP TS 29.244)
/// ```text
/// +-----------+-----------+-----------+-----------+
/// |         Linked URR ID (32 bits)              |
/// +-----------+-----------+-----------+-----------+
/// ```
///
/// # Example
/// ```
/// use rs_pfcp::ie::linked_urr_id::LinkedUrrId;
///
/// let linked_urr = LinkedUrrId::new(42);
/// let marshaled = linked_urr.marshal();
/// let unmarshaled = LinkedUrrId::unmarshal(&marshaled).unwrap();
/// assert_eq!(linked_urr, unmarshaled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkedUrrId {
    /// The URR ID that this URR is linked to
    pub id: u32,
}

impl LinkedUrrId {
    /// Creates a new Linked URR ID.
    ///
    /// # Arguments
    /// * `id` - The URR ID to link to (u32)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::linked_urr_id::LinkedUrrId;
    ///
    /// let linked_urr = LinkedUrrId::new(1);
    /// assert_eq!(linked_urr.id, 1);
    /// ```
    pub fn new(id: u32) -> Self {
        LinkedUrrId { id }
    }

    /// Marshals the Linked URR ID into a byte vector.
    ///
    /// # Returns
    /// A 4-byte vector in big-endian (network byte order).
    pub fn marshal(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a Linked URR ID.
    ///
    /// # Arguments
    /// * `payload` - Byte slice containing the Linked URR ID
    ///
    /// # Returns
    /// A `LinkedUrrId` instance or an error if unmarshaling fails.
    ///
    /// # Errors
    /// Returns `io::Error` if:
    /// - The buffer is too short (< 4 bytes)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::linked_urr_id::LinkedUrrId;
    ///
    /// let data = vec![0x00, 0x00, 0x00, 0x2A]; // 42 in big-endian
    /// let linked_urr = LinkedUrrId::unmarshal(&data).unwrap();
    /// assert_eq!(linked_urr.id, 42);
    /// ```
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Linked URR ID requires 4 bytes (u32), got {}",
                    payload.len()
                ),
            ));
        }
        Ok(LinkedUrrId {
            id: u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]),
        })
    }

    /// Wraps the Linked URR ID in a generic IE.
    ///
    /// # Returns
    /// An `Ie` with type `LinkedUrrId` and the marshaled payload.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::LinkedUrrId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_urr_id_new() {
        let linked_urr = LinkedUrrId::new(42);
        assert_eq!(linked_urr.id, 42);
    }

    #[test]
    fn test_linked_urr_id_marshal_unmarshal() {
        let linked_urr = LinkedUrrId::new(0x12345678);
        let marshaled = linked_urr.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = LinkedUrrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, 0x12345678);
    }

    #[test]
    fn test_linked_urr_id_unmarshal_empty() {
        let result = LinkedUrrId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 0"));
    }

    #[test]
    fn test_linked_urr_id_unmarshal_too_short() {
        // Test with 1, 2, and 3 bytes
        for len in 1..4 {
            let data = vec![0xFF; len];
            let result = LinkedUrrId::unmarshal(&data);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::InvalidData);
            assert!(err.to_string().contains("requires 4 bytes"));
            assert!(err.to_string().contains(&format!("got {}", len)));
        }
    }

    #[test]
    fn test_linked_urr_id_round_trip() {
        let test_ids = vec![0, 1, 0xFFFFFFFF, 0x12345678, 0xABCDEF00];
        for id in test_ids {
            let linked_urr = LinkedUrrId::new(id);
            let marshaled = linked_urr.marshal();
            let unmarshaled = LinkedUrrId::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.id, id);
        }
    }

    #[test]
    fn test_linked_urr_id_to_ie() {
        let linked_urr = LinkedUrrId::new(0x11223344);
        let ie = linked_urr.to_ie();
        assert_eq!(ie.ie_type, IeType::LinkedUrrId);
        assert_eq!(ie.payload, vec![0x11, 0x22, 0x33, 0x44]);

        let unmarshaled = LinkedUrrId::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.id, 0x11223344);
    }

    #[test]
    fn test_linked_urr_id_edge_cases() {
        // Zero value
        let linked_urr = LinkedUrrId::new(0);
        let marshaled = linked_urr.marshal();
        let unmarshaled = LinkedUrrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, 0);

        // Max value (u32::MAX = 4294967295)
        let linked_urr = LinkedUrrId::new(u32::MAX);
        let marshaled = linked_urr.marshal();
        let unmarshaled = LinkedUrrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, u32::MAX);
    }

    #[test]
    fn test_linked_urr_id_copy_trait() {
        let linked_urr1 = LinkedUrrId::new(100);
        let linked_urr2 = linked_urr1; // Should copy, not move
        assert_eq!(linked_urr1.id, linked_urr2.id);
        assert_eq!(linked_urr1, linked_urr2);
    }
}
