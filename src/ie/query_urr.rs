// src/ie/query_urr.rs

//! Query URR Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.77, the Query URR IE is used to request
//! usage reporting information for a specific URR (Usage Reporting Rule).

use crate::ie::urr_id::UrrId;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Query URR IE.
///
/// This IE is used in Session Modification Request messages to query the
/// current usage reporting state for a specific URR without modifying it.
///
/// # Structure
///
/// - URR ID (mandatory) - Identifies the URR to query
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::query_urr::QueryUrr;
/// use rs_pfcp::ie::urr_id::UrrId;
///
/// // Create a Query URR IE
/// let urr_id = UrrId::new(1);
/// let query_urr = QueryUrr::new(urr_id);
///
/// // Marshal and unmarshal
/// let marshaled = query_urr.marshal();
/// let unmarshaled = QueryUrr::unmarshal(&marshaled).unwrap();
/// assert_eq!(unmarshaled, query_urr);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryUrr {
    /// URR ID (mandatory)
    pub urr_id: UrrId,
}

impl QueryUrr {
    /// Creates a new Query URR IE.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The URR identifier to query
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::query_urr::QueryUrr;
    /// use rs_pfcp::ie::urr_id::UrrId;
    ///
    /// let urr_id = UrrId::new(5);
    /// let query_urr = QueryUrr::new(urr_id);
    /// assert_eq!(query_urr.urr_id.id, 5);
    /// ```
    pub fn new(urr_id: UrrId) -> Self {
        QueryUrr { urr_id }
    }

    /// Marshals the Query URR into a byte vector.
    ///
    /// The Query URR contains only the URR ID (4 bytes).
    pub fn marshal(&self) -> Vec<u8> {
        self.urr_id.marshal().to_vec()
    }

    /// Unmarshals a byte slice into a Query URR IE.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte slice to unmarshal (must be 4 bytes for URR ID)
    ///
    /// # Returns
    ///
    /// Returns `Ok(QueryUrr)` on success, or an error if the data is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The data is not exactly 4 bytes (URR ID length)
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(QueryUrr {
            urr_id: UrrId::unmarshal(data)?,
        })
    }

    /// Wraps the Query URR in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::query_urr::QueryUrr;
    /// use rs_pfcp::ie::urr_id::UrrId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let query_urr = QueryUrr::new(UrrId::new(10));
    /// let ie = query_urr.to_ie();
    /// assert_eq!(ie.ie_type, IeType::QueryUrr);
    /// ```
    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::QueryUrr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_urr_marshal_unmarshal() {
        let urr_id = UrrId::new(0x12345678);
        let query_urr = QueryUrr::new(urr_id);

        let marshaled = query_urr.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = QueryUrr::unmarshal(&marshaled).unwrap();
        assert_eq!(query_urr, unmarshaled);
        assert_eq!(unmarshaled.urr_id.id, 0x12345678);
    }

    #[test]
    fn test_query_urr_to_ie() {
        let urr_id = UrrId::new(1234);
        let query_urr = QueryUrr::new(urr_id);

        let ie = query_urr.to_ie();
        assert_eq!(ie.ie_type, IeType::QueryUrr);
        assert_eq!(ie.payload.len(), 4);

        let unmarshaled = QueryUrr::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.urr_id.id, 1234);
    }

    #[test]
    fn test_query_urr_unmarshal_invalid_length() {
        // Too short
        let result = QueryUrr::unmarshal(&[0x00]);
        assert!(result.is_err());

        // Too long
        let result = QueryUrr::unmarshal(&[0x00, 0x00, 0x00, 0x00, 0x00]);
        assert!(result.is_err());
    }

    #[test]
    fn test_query_urr_unmarshal_empty() {
        let result = QueryUrr::unmarshal(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_query_urr_round_trip() {
        let test_ids = vec![0, 1, 100, 0x7FFFFFFF, 0xFFFFFFFF];
        for id in test_ids {
            let urr_id = UrrId::new(id);
            let query_urr = QueryUrr::new(urr_id);

            let marshaled = query_urr.marshal();
            let unmarshaled = QueryUrr::unmarshal(&marshaled).unwrap();

            assert_eq!(unmarshaled.urr_id.id, id);
        }
    }

    #[test]
    fn test_query_urr_equality() {
        let query_urr1 = QueryUrr::new(UrrId::new(100));
        let query_urr2 = QueryUrr::new(UrrId::new(100));
        let query_urr3 = QueryUrr::new(UrrId::new(200));

        assert_eq!(query_urr1, query_urr2);
        assert_ne!(query_urr1, query_urr3);
    }

    #[test]
    fn test_query_urr_specific_values() {
        // Test minimum URR ID
        let query_urr_min = QueryUrr::new(UrrId::new(0));
        let marshaled_min = query_urr_min.marshal();
        assert_eq!(marshaled_min, vec![0x00, 0x00, 0x00, 0x00]);

        // Test maximum URR ID
        let query_urr_max = QueryUrr::new(UrrId::new(0xFFFFFFFF));
        let marshaled_max = query_urr_max.marshal();
        assert_eq!(marshaled_max, vec![0xFF, 0xFF, 0xFF, 0xFF]);

        // Round trip both
        let unmarshaled_min = QueryUrr::unmarshal(&marshaled_min).unwrap();
        let unmarshaled_max = QueryUrr::unmarshal(&marshaled_max).unwrap();

        assert_eq!(unmarshaled_min, query_urr_min);
        assert_eq!(unmarshaled_max, query_urr_max);
    }
}
