//! Iterator infrastructure for unified IE access patterns.
//!
//! This module provides iterators for accessing Information Elements (IEs) in PFCP messages
//! in a consistent way, regardless of whether the IE appears 0, 1, or many times.
//!
//! # Overview
//!
//! PFCP messages store IEs in three different patterns:
//! - **Single mandatory**: `pub node_id: Ie`
//! - **Optional**: `pub pdn_type: Option<Ie>`
//! - **Multiple**: `pub create_pdrs: Vec<Ie>`
//!
//! The iterator-based API provides a unified way to access all patterns:
//!
//! ```rust
//! use rs_pfcp::message::{Message, SessionEstablishmentRequest};
//! use rs_pfcp::ie::IeType;
//!
//! # fn example(msg: &SessionEstablishmentRequest) {
//! // Single mandatory IE
//! if let Some(node_id) = msg.ies(IeType::NodeId).next() {
//!     println!("Node ID found");
//! }
//!
//! // Optional IE
//! match msg.ies(IeType::PdnType).next() {
//!     Some(pdn) => println!("PDN Type present"),
//!     None => println!("No PDN Type"),
//! }
//!
//! // Multiple IEs
//! for pdr in msg.ies(IeType::CreatePdr) {
//!     println!("Processing PDR");
//! }
//!
//! // Count IEs
//! let pdr_count = msg.ies(IeType::CreatePdr).count();
//! # }
//! ```

use crate::ie::Ie;
use crate::ie::IeType;

/// Iterator over Information Elements of a specific type in a message.
///
/// This iterator provides a unified way to access IEs regardless of their storage pattern
/// (single mandatory, optional, or multiple). It's zero-cost and optimizes to direct access.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::message::{Message, HeartbeatRequest};
/// use rs_pfcp::ie::{Ie, IeType, recovery_time_stamp::RecoveryTimeStamp};
/// use std::time::SystemTime;
///
/// let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
/// let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
/// let request = HeartbeatRequest::new(123, ts_ie, None, vec![]);
///
/// // Iterate over all Recovery Time Stamp IEs (there's one)
/// let mut iter = request.ies(IeType::RecoveryTimeStamp);
/// assert!(iter.next().is_some());
/// assert!(iter.next().is_none());
/// ```
#[derive(Debug)]
pub struct IeIter<'a> {
    ie_type: IeType,
    state: IeIterState<'a>,
}

/// Internal state of the IE iterator.
#[derive(Debug)]
enum IeIterState<'a> {
    /// Iterating over a single IE (mandatory or optional).
    ///
    /// This variant is used when the IE is stored directly or as `Option<Ie>`.
    /// It will yield at most one item.
    Single(Option<&'a Ie>),

    /// Iterating over multiple IEs (vector).
    ///
    /// This variant is used when the IE is stored as `Vec<Ie>`.
    /// It will yield zero or more items.
    Multiple(std::slice::Iter<'a, Ie>),

    /// Generic fallback: searching through all IEs.
    ///
    /// This variant is used when the specific IE storage location is unknown.
    /// It searches through a slice of all IEs in the message.
    Generic { all_ies: &'a [Ie], position: usize },
}

impl<'a> Iterator for IeIter<'a> {
    type Item = &'a Ie;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            IeIterState::Single(opt) => opt.take(),
            IeIterState::Multiple(iter) => iter.next(),
            IeIterState::Generic { all_ies, position } => {
                while *position < all_ies.len() {
                    let ie = &all_ies[*position];
                    *position += 1;
                    if ie.ie_type == self.ie_type {
                        return Some(ie);
                    }
                }
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.state {
            IeIterState::Single(Some(_)) => (1, Some(1)),
            IeIterState::Single(None) => (0, Some(0)),
            IeIterState::Multiple(iter) => iter.size_hint(),
            IeIterState::Generic { all_ies, position } => {
                // We don't know how many match, so give conservative bounds
                let remaining = all_ies.len() - *position;
                (0, Some(remaining))
            }
        }
    }
}

impl<'a> IeIter<'a> {
    /// Create an iterator for a single IE (mandatory or optional).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_pfcp::message::ie_iter::IeIter;
    /// use rs_pfcp::ie::{Ie, IeType};
    ///
    /// let ie = Ie::new(IeType::Cause, vec![0x01]);
    ///
    /// // Mandatory IE (always present)
    /// let mut iter = IeIter::single(Some(&ie), IeType::Cause);
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_none());
    ///
    /// // Optional IE (not present)
    /// let mut iter = IeIter::single(None, IeType::PdnType);
    /// assert!(iter.next().is_none());
    /// ```
    pub fn single(ie: Option<&'a Ie>, ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Single(ie),
        }
    }

    /// Create an iterator for multiple IEs (vector).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_pfcp::message::ie_iter::IeIter;
    /// use rs_pfcp::ie::{Ie, IeType};
    ///
    /// let ies = vec![
    ///     Ie::new(IeType::CreatePdr, vec![1]),
    ///     Ie::new(IeType::CreatePdr, vec![2]),
    ///     Ie::new(IeType::CreatePdr, vec![3]),
    /// ];
    ///
    /// let mut iter = IeIter::multiple(&ies, IeType::CreatePdr);
    /// assert_eq!(iter.count(), 3);
    /// ```
    pub fn multiple(ies: &'a [Ie], ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Multiple(ies.iter()),
        }
    }

    /// Create an iterator that searches through all IEs.
    ///
    /// This is a generic fallback used when the specific storage location
    /// for an IE type is not known. It linearly searches through all IEs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_pfcp::message::ie_iter::IeIter;
    /// use rs_pfcp::ie::{Ie, IeType};
    ///
    /// let ies = vec![
    ///     Ie::new(IeType::Cause, vec![1]),
    ///     Ie::new(IeType::CreatePdr, vec![2]),
    ///     Ie::new(IeType::Cause, vec![3]),
    ///     Ie::new(IeType::CreateFar, vec![4]),
    /// ];
    ///
    /// // Find all Cause IEs
    /// let causes: Vec<_> = IeIter::generic(&ies, IeType::Cause).collect();
    /// assert_eq!(causes.len(), 2);
    /// ```
    pub fn generic(all_ies: &'a [Ie], ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Generic {
                all_ies,
                position: 0,
            },
        }
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ie_iterator_some() {
        let ie = Ie::new(IeType::Cause, vec![0x01]);
        let mut iter = IeIter::single(Some(&ie), IeType::Cause);

        assert_eq!(iter.next().map(|i| &i.payload), Some(&vec![0x01]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_ie_iterator_none() {
        let mut iter = IeIter::single(None, IeType::PdnType);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multiple_ie_iterator_empty() {
        let ies: Vec<Ie> = vec![];
        let mut iter = IeIter::multiple(&ies, IeType::CreatePdr);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multiple_ie_iterator_three() {
        let ies = vec![
            Ie::new(IeType::CreatePdr, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
            Ie::new(IeType::CreatePdr, vec![3]),
        ];

        let mut iter = IeIter::multiple(&ies, IeType::CreatePdr);

        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(1));
        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(2));
        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_generic_ie_iterator_filters_correctly() {
        let ies = vec![
            Ie::new(IeType::Cause, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
            Ie::new(IeType::Cause, vec![3]),
            Ie::new(IeType::CreateFar, vec![4]),
            Ie::new(IeType::Cause, vec![5]),
        ];

        let collected: Vec<_> = IeIter::generic(&ies, IeType::Cause)
            .map(|ie| ie.payload[0])
            .collect();

        assert_eq!(collected, vec![1, 3, 5]);
    }

    #[test]
    fn test_generic_ie_iterator_no_matches() {
        let ies = vec![
            Ie::new(IeType::Cause, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
        ];

        let collected: Vec<_> = IeIter::generic(&ies, IeType::NodeId).collect();

        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn test_iterator_count() {
        let ies = vec![
            Ie::new(IeType::CreatePdr, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
            Ie::new(IeType::CreatePdr, vec![3]),
        ];

        let count = IeIter::multiple(&ies, IeType::CreatePdr).count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_iterator_collect() {
        let ies = vec![
            Ie::new(IeType::CreateFar, vec![10]),
            Ie::new(IeType::CreateFar, vec![20]),
        ];

        let collected: Vec<_> = IeIter::multiple(&ies, IeType::CreateFar).collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].payload[0], 10);
        assert_eq!(collected[1].payload[0], 20);
    }

    #[test]
    fn test_size_hint_single_some() {
        let ie = Ie::new(IeType::Cause, vec![0x01]);
        let iter = IeIter::single(Some(&ie), IeType::Cause);

        assert_eq!(iter.size_hint(), (1, Some(1)));
    }

    #[test]
    fn test_size_hint_single_none() {
        let iter = IeIter::single(None, IeType::PdnType);

        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_multiple() {
        let ies = vec![
            Ie::new(IeType::CreatePdr, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
        ];

        let iter = IeIter::multiple(&ies, IeType::CreatePdr);
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_chaining_iterator_methods() {
        let ies = vec![
            Ie::new(IeType::CreateQer, vec![5]),
            Ie::new(IeType::CreateQer, vec![10]),
            Ie::new(IeType::CreateQer, vec![15]),
        ];

        // Use iterator combinators
        let sum: u8 = IeIter::multiple(&ies, IeType::CreateQer)
            .map(|ie| ie.payload[0])
            .sum();

        assert_eq!(sum, 30);
    }
}
