//! Type-safe newtype wrappers for PFCP protocol identifiers.
//!
//! This module provides newtype wrappers that prevent accidental argument swapping
//! at compile time. For example, mixing up `seid` and `sequence_number` parameters
//! is a common source of bugs that these types eliminate.
//!
//! # Types
//!
//! - [`Seid`] - Session Endpoint Identifier (64-bit)
//! - [`SequenceNumber`] - Message sequence number (24-bit, stored as u32)
//! - [`Teid`] - Tunnel Endpoint Identifier (32-bit)
//!
//! # Examples
//!
//! ```rust
//! use rs_pfcp::types::{Seid, SequenceNumber, Teid};
//!
//! // Create from primitives
//! let seid = Seid(0x123456789ABCDEF0);
//! let seq = SequenceNumber::new(42);
//! let teid = Teid(0x12345678);
//!
//! // Convert back to primitives
//! let seid_val: u64 = seid.into();
//! let seq_val: u32 = seq.into();
//! let teid_val: u32 = teid.into();
//!
//! // Use From trait
//! let seid2: Seid = 100u64.into();
//! let seq2: SequenceNumber = 50u32.into();
//! let teid2: Teid = 200u32.into();
//!
//! // Access inner value via Deref
//! assert_eq!(*seid, 0x123456789ABCDEF0u64);
//! ```

use std::fmt;
use std::ops::Deref;

// ============================================================================
// Seid - Session Endpoint Identifier
// ============================================================================

/// Session Endpoint Identifier (SEID) - 64-bit identifier.
///
/// The SEID uniquely identifies a PFCP session on a given node.
/// Per 3GPP TS 29.244, the SEID is present in session-related messages
/// to identify the PFCP session context.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::types::Seid;
///
/// let seid = Seid(0x123456789ABCDEF0);
/// assert_eq!(*seid, 0x123456789ABCDEF0);
///
/// // From/Into conversions
/// let seid: Seid = 42u64.into();
/// let value: u64 = seid.into();
/// assert_eq!(value, 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Seid(pub u64);

impl Seid {
    /// Creates a new SEID with the given value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the inner u64 value.
    #[inline]
    pub const fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Seid {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Seid> for u64 {
    #[inline]
    fn from(seid: Seid) -> Self {
        seid.0
    }
}

impl Deref for Seid {
    type Target = u64;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Seid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:016X}", self.0)
    }
}

impl fmt::LowerHex for Seid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Seid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

// ============================================================================
// SequenceNumber - 24-bit Message Sequence Number
// ============================================================================

/// Sequence Number - 24-bit message sequence number (stored as u32).
///
/// Per 3GPP TS 29.244 Section 5.1, the sequence number is a 24-bit field
/// used to match request and response messages. Valid range: 0 to 0x00FFFFFF.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::types::SequenceNumber;
///
/// // Create with masking to 24 bits
/// let seq = SequenceNumber::new(0x123456);
/// assert_eq!(*seq, 0x123456);
///
/// // Values larger than 24 bits are masked
/// let seq = SequenceNumber::new(0xFF123456);
/// assert_eq!(*seq, 0x123456);
///
/// // Increment with wrap-around
/// let seq = SequenceNumber::new(SequenceNumber::MAX);
/// let next = seq.next();
/// assert_eq!(*next, 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SequenceNumber(pub u32);

impl SequenceNumber {
    /// Maximum value for a 24-bit sequence number.
    pub const MAX: u32 = 0x00FFFFFF;

    /// Creates a new sequence number, masking to 24 bits.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value & Self::MAX)
    }

    /// Returns the inner u32 value.
    #[inline]
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Returns the next sequence number with wrap-around at MAX.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_pfcp::types::SequenceNumber;
    ///
    /// let seq = SequenceNumber::new(100);
    /// assert_eq!(*seq.next(), 101);
    ///
    /// let max_seq = SequenceNumber::new(SequenceNumber::MAX);
    /// assert_eq!(*max_seq.next(), 0);
    /// ```
    #[inline]
    pub const fn next(&self) -> Self {
        Self((self.0 + 1) & Self::MAX)
    }

    /// Checks if this sequence number is within a valid 24-bit range.
    ///
    /// Always returns true since the constructor masks to 24 bits.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 <= Self::MAX
    }
}

impl From<u32> for SequenceNumber {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<SequenceNumber> for u32 {
    #[inline]
    fn from(seq: SequenceNumber) -> Self {
        seq.0
    }
}

impl Deref for SequenceNumber {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Teid - Tunnel Endpoint Identifier
// ============================================================================

/// Tunnel Endpoint Identifier (TEID) - 32-bit identifier.
///
/// The TEID identifies a GTP-U tunnel endpoint. It is used in F-TEID IEs
/// to specify tunnel endpoints for user plane data forwarding.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::types::Teid;
///
/// let teid = Teid(0x12345678);
/// assert_eq!(*teid, 0x12345678);
///
/// // From/Into conversions
/// let teid: Teid = 42u32.into();
/// let value: u32 = teid.into();
/// assert_eq!(value, 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Teid(pub u32);

impl Teid {
    /// Creates a new TEID with the given value.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the inner u32 value.
    #[inline]
    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Teid {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Teid> for u32 {
    #[inline]
    fn from(teid: Teid) -> Self {
        teid.0
    }
}

impl Deref for Teid {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Teid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}

impl fmt::LowerHex for Teid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Teid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Seid tests
    mod seid_tests {
        use super::*;

        #[test]
        fn test_seid_new() {
            let seid = Seid::new(0x123456789ABCDEF0);
            assert_eq!(seid.value(), 0x123456789ABCDEF0);
        }

        #[test]
        fn test_seid_default() {
            let seid = Seid::default();
            assert_eq!(*seid, 0);
        }

        #[test]
        fn test_seid_from_u64() {
            let seid: Seid = 42u64.into();
            assert_eq!(*seid, 42);
        }

        #[test]
        fn test_seid_into_u64() {
            let seid = Seid(100);
            let value: u64 = seid.into();
            assert_eq!(value, 100);
        }

        #[test]
        fn test_seid_deref() {
            let seid = Seid(0xDEADBEEF);
            assert_eq!(*seid, 0xDEADBEEF);
        }

        #[test]
        fn test_seid_display() {
            let seid = Seid(0x123456789ABCDEF0);
            assert_eq!(format!("{}", seid), "0x123456789ABCDEF0");
        }

        #[test]
        fn test_seid_equality() {
            let seid1 = Seid(100);
            let seid2 = Seid(100);
            let seid3 = Seid(200);
            assert_eq!(seid1, seid2);
            assert_ne!(seid1, seid3);
        }

        #[test]
        fn test_seid_clone() {
            let seid = Seid(42);
            let cloned = seid;
            assert_eq!(seid, cloned);
        }

        #[test]
        fn test_seid_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(Seid(1));
            set.insert(Seid(2));
            set.insert(Seid(1));
            assert_eq!(set.len(), 2);
        }
    }

    // SequenceNumber tests
    mod sequence_number_tests {
        use super::*;

        #[test]
        fn test_sequence_number_new() {
            let seq = SequenceNumber::new(42);
            assert_eq!(seq.value(), 42);
        }

        #[test]
        fn test_sequence_number_max() {
            assert_eq!(SequenceNumber::MAX, 0x00FFFFFF);
        }

        #[test]
        fn test_sequence_number_masking() {
            let seq = SequenceNumber::new(0xFF123456);
            assert_eq!(*seq, 0x123456);
        }

        #[test]
        fn test_sequence_number_default() {
            let seq = SequenceNumber::default();
            assert_eq!(*seq, 0);
        }

        #[test]
        fn test_sequence_number_next() {
            let seq = SequenceNumber::new(100);
            assert_eq!(*seq.next(), 101);
        }

        #[test]
        fn test_sequence_number_next_wrap() {
            let seq = SequenceNumber::new(SequenceNumber::MAX);
            assert_eq!(*seq.next(), 0);
        }

        #[test]
        fn test_sequence_number_from_u32() {
            let seq: SequenceNumber = 50u32.into();
            assert_eq!(*seq, 50);
        }

        #[test]
        fn test_sequence_number_into_u32() {
            let seq = SequenceNumber::new(75);
            let value: u32 = seq.into();
            assert_eq!(value, 75);
        }

        #[test]
        fn test_sequence_number_deref() {
            let seq = SequenceNumber::new(999);
            assert_eq!(*seq, 999);
        }

        #[test]
        fn test_sequence_number_display() {
            let seq = SequenceNumber::new(12345);
            assert_eq!(format!("{}", seq), "12345");
        }

        #[test]
        fn test_sequence_number_ordering() {
            let seq1 = SequenceNumber::new(10);
            let seq2 = SequenceNumber::new(20);
            assert!(seq1 < seq2);
            assert!(seq2 > seq1);
        }

        #[test]
        fn test_sequence_number_is_valid() {
            let seq = SequenceNumber::new(100);
            assert!(seq.is_valid());

            let seq_max = SequenceNumber::new(SequenceNumber::MAX);
            assert!(seq_max.is_valid());
        }
    }

    // Teid tests
    mod teid_tests {
        use super::*;

        #[test]
        fn test_teid_new() {
            let teid = Teid::new(0x12345678);
            assert_eq!(teid.value(), 0x12345678);
        }

        #[test]
        fn test_teid_default() {
            let teid = Teid::default();
            assert_eq!(*teid, 0);
        }

        #[test]
        fn test_teid_from_u32() {
            let teid: Teid = 42u32.into();
            assert_eq!(*teid, 42);
        }

        #[test]
        fn test_teid_into_u32() {
            let teid = Teid(100);
            let value: u32 = teid.into();
            assert_eq!(value, 100);
        }

        #[test]
        fn test_teid_deref() {
            let teid = Teid(0xDEADBEEF);
            assert_eq!(*teid, 0xDEADBEEF);
        }

        #[test]
        fn test_teid_display() {
            let teid = Teid(0x12345678);
            assert_eq!(format!("{}", teid), "0x12345678");
        }

        #[test]
        fn test_teid_equality() {
            let teid1 = Teid(100);
            let teid2 = Teid(100);
            let teid3 = Teid(200);
            assert_eq!(teid1, teid2);
            assert_ne!(teid1, teid3);
        }

        #[test]
        fn test_teid_clone() {
            let teid = Teid(42);
            let cloned = teid;
            assert_eq!(teid, cloned);
        }

        #[test]
        fn test_teid_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(Teid(1));
            set.insert(Teid(2));
            set.insert(Teid(1));
            assert_eq!(set.len(), 2);
        }
    }
}
