//! # rs-pfcp
//!
//! A high-performance Rust implementation of the PFCP (Packet Forwarding Control Protocol)
//! for 5G networks, providing 100% compliance with 3GPP TS 29.244 Release 18 specification.
//!
//! ## What is PFCP?
//!
//! PFCP is the critical communication protocol between Control Plane and User Plane functions in 5G networks:
//! - **SMF (Session Management Function)** ↔ **UPF (User Plane Function)**
//! - Manages packet forwarding rules, traffic steering, and usage reporting
//! - Essential for 5G service orchestration, QoS enforcement, and network slicing
//!
//! ## Quick Start
//!
//! ```rust
//! use rs_pfcp::message::{SessionEstablishmentRequest, SessionEstablishmentRequestBuilder};
//! use rs_pfcp::ie::{NodeId, Cause, CauseValue};
//!
//! // Create a session establishment request
//! let request = SessionEstablishmentRequestBuilder::new(session_id, sequence_number)
//!     .node_id(NodeId::from_ipv4("10.0.0.1".parse()?))
//!     .fseid(fseid_ie)
//!     .create_pdrs(vec![create_pdr_ie])
//!     .create_fars(vec![create_far_ie])
//!     .build()?;
//!
//! // Serialize to bytes for network transmission
//! let bytes = request.marshal();
//!
//! // Parse received messages
//! let parsed_msg = rs_pfcp::message::parse(&bytes)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Module Organization
//!
//! - [`ie`] - Information Elements (IEs) as defined in 3GPP TS 29.244
//! - [`message`] - PFCP message types for session and association management

pub mod ie;
pub mod message;
