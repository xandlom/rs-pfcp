//! PFCP message comparison functionality.
//!
//! This module provides tools for comparing PFCP messages with configurable options,
//! enabling use cases like testing, debugging, validation, and compliance auditing.
//!
//! # Overview
//!
//! The comparison system uses a builder pattern to provide flexible, type-safe
//! configuration of comparison behavior:
//!
//! ```rust,no_run
//! use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
//!
//! let result = MessageComparator::new(msg1, msg2)
//!     .ignore_sequence()
//!     .ignore_timestamps()
//!     .compare()?;
//!
//! assert!(result.is_match());
//! # Ok(())
//! # }
//! ```
//!
//! # Comparison Levels
//!
//! ## Exact Comparison
//! Default behavior - all fields must match exactly:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
//! let result = MessageComparator::new(msg1, msg2)
//!     .strict_mode()
//!     .compare()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Semantic Comparison
//! Ignore transient fields, focus on functional equivalence:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
//! let result = MessageComparator::new(msg1, msg2)
//!     .test_mode()  // Ignores sequence, timestamps, etc.
//!     .compare()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Focused Comparison
//! Compare only specific Information Elements:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # use rs_pfcp::ie::IeType;
//! # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
//! let result = MessageComparator::new(msg1, msg2)
//!     .focus_on_ie_types(&[IeType::CreatePdr, IeType::CreateFar])
//!     .compare()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Diff Generation
//!
//! Generate detailed differences between messages:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
//! let diff = MessageComparator::new(msg1, msg2)
//!     .with_detailed_diff()
//!     .diff()?;
//!
//! println!("Differences found: {}", diff.len());
//! println!("{}", diff);  // Human-readable output
//! # Ok(())
//! # }
//! ```
//!
//! # Use Cases
//!
//! ## Testing
//! Verify round-trip marshal/unmarshal operations:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::{Message, heartbeat_request::HeartbeatRequest};
//! # fn example(original: &HeartbeatRequest) -> std::io::Result<()> {
//! let bytes = original.marshal();
//! let parsed = HeartbeatRequest::unmarshal(&bytes)?;
//!
//! let result = MessageComparator::new(original, &parsed)
//!     .test_mode()
//!     .compare()?;
//!
//! assert!(result.is_match(), "Round-trip failed");
//! # Ok(())
//! # }
//! ```
//!
//! ## Debugging
//! Find differences between expected and actual messages:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # fn example(expected: &dyn Message, actual: &dyn Message) -> std::io::Result<()> {
//! let diff = MessageComparator::new(expected, actual)
//!     .include_payload_in_diff()
//!     .diff()?;
//!
//! for difference in &diff.differences {
//!     println!("Mismatch: {:?}", difference);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Validation
//! Ensure messages contain required IEs:
//! ```rust,no_run
//! # use rs_pfcp::comparison::MessageComparator;
//! # use rs_pfcp::message::Message;
//! # use rs_pfcp::comparison::OptionalIeMode;
//! # fn example(reference: &dyn Message, msg: &dyn Message) -> std::io::Result<()> {
//! let result = MessageComparator::new(reference, msg)
//!     .optional_ie_mode(OptionalIeMode::RequireLeft)
//!     .semantic_mode()
//!     .matches()?;
//!
//! assert!(result, "Required IEs missing");
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod diff;
pub mod options;
pub mod result;

pub use builder::MessageComparator;
pub use diff::{Difference, HeaderField, MessageDiff};
pub use options::{ComparisonOptions, IeMultiplicityMode, OptionalIeMode};
pub use result::{
    ComparisonResult, ComparisonStats, HeaderMatch, IeMatch, IeMatchType, IeMismatch,
    MismatchReason,
};
