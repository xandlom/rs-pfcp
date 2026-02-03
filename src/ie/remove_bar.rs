//! Remove BAR IE implementation.
//!
//! Per 3GPP TS 29.244 Section 8.2.89, the Remove BAR IE is used to remove
//! a Buffering Action Rule from a PFCP session.

use crate::error::PfcpError;
use crate::ie::bar_id::BarId;
use crate::ie::{Ie, IeType};

/// Remove BAR Information Element.
///
/// Used in PFCP Session Modification Request to remove an existing
/// Buffering Action Rule identified by BAR ID.
///
/// # Structure (per 3GPP TS 29.244)
/// ```text
/// +-----------+
/// |  BAR ID   |  (1 byte)
/// +-----------+
/// ```
///
/// # Example
/// ```
/// use rs_pfcp::ie::remove_bar::RemoveBar;
/// use rs_pfcp::ie::bar_id::BarId;
///
/// let remove_bar = RemoveBar::new(BarId::new(42));
/// let marshaled = remove_bar.marshal();
/// let unmarshaled = RemoveBar::unmarshal(&marshaled).unwrap();
/// assert_eq!(remove_bar, unmarshaled);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveBar {
    /// BAR ID of the Buffering Action Rule to remove
    pub bar_id: BarId,
}

impl RemoveBar {
    /// Creates a new Remove BAR IE.
    ///
    /// # Arguments
    /// * `bar_id` - The BAR ID to remove
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::remove_bar::RemoveBar;
    /// use rs_pfcp::ie::bar_id::BarId;
    ///
    /// let remove_bar = RemoveBar::new(BarId::new(1));
    /// ```
    pub fn new(bar_id: BarId) -> Self {
        RemoveBar { bar_id }
    }

    /// Marshals the Remove BAR IE to bytes.
    ///
    /// # Returns
    /// A vector containing the serialized BAR ID (1 byte).
    pub fn marshal(&self) -> Vec<u8> {
        self.bar_id.marshal().to_vec()
    }

    /// Converts to a generic IE.
    ///
    /// # Returns
    /// An `Ie` with type `RemoveBar` and the marshaled payload.
    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemoveBar, self.marshal())
    }

    /// Unmarshals a Remove BAR IE from bytes.
    ///
    /// # Arguments
    /// * `data` - The byte slice containing the BAR ID
    ///
    /// # Returns
    /// A `RemoveBar` instance or an error if unmarshaling fails.
    ///
    /// # Errors
    /// Returns `PfcpError` if:
    /// - The buffer is too short (< 1 byte)
    /// - The BAR ID cannot be parsed
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(RemoveBar {
            bar_id: BarId::unmarshal(data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_bar_new() {
        let bar_id = BarId::new(42);
        let remove_bar = RemoveBar::new(bar_id.clone());
        assert_eq!(remove_bar.bar_id, bar_id);
    }

    #[test]
    fn test_remove_bar_marshal_unmarshal() {
        let remove_bar = RemoveBar::new(BarId::new(123));
        let marshaled = remove_bar.marshal();
        assert_eq!(marshaled, vec![123]);

        let unmarshaled = RemoveBar::unmarshal(&marshaled).unwrap();
        assert_eq!(remove_bar, unmarshaled);
    }

    #[test]
    fn test_remove_bar_round_trip() {
        let original = RemoveBar::new(BarId::new(255));
        let marshaled = original.marshal();
        let unmarshaled = RemoveBar::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_remove_bar_to_ie() {
        let ie = RemoveBar::new(BarId::new(100)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveBar);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 100);
    }

    #[test]
    fn test_remove_bar_unmarshal_empty_buffer() {
        // Empty payload
        assert!(RemoveBar::unmarshal(&[]).is_err());
    }

    #[test]
    fn test_remove_bar_edge_cases() {
        // Zero value
        let remove_bar = RemoveBar::new(BarId::new(0));
        let marshaled = remove_bar.marshal();
        let unmarshaled = RemoveBar::unmarshal(&marshaled).unwrap();
        assert_eq!(remove_bar, unmarshaled);

        // Max value (u8::MAX = 255)
        let remove_bar = RemoveBar::new(BarId::new(255));
        let marshaled = remove_bar.marshal();
        let unmarshaled = RemoveBar::unmarshal(&marshaled).unwrap();
        assert_eq!(remove_bar, unmarshaled);
    }
}
