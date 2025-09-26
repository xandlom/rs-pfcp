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
//! # use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
//! # use rs_pfcp::message::Message;
//! # use rs_pfcp::ie::node_id::NodeId;
//! # use rs_pfcp::ie::fseid::Fseid;
//! # use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
//! # use rs_pfcp::ie::create_far::CreateFar;
//! # use rs_pfcp::ie::create_qer::CreateQerBuilder;
//! # use rs_pfcp::ie::f_teid::FteidBuilder;
//! # use rs_pfcp::ie::pdr_id::PdrId;
//! # use rs_pfcp::ie::precedence::Precedence;
//! # use rs_pfcp::ie::pdi::{Pdi, PdiBuilder};
//! # use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
//! # use rs_pfcp::ie::far_id::FarId;
//! # use rs_pfcp::ie::qer_id::QerId;
//! # use rs_pfcp::ie::apply_action::ApplyAction;
//! # use rs_pfcp::ie::{Ie, IeType};
//! # use std::net::Ipv4Addr;
//!
//! // Create F-TEID using new builder pattern
//! let fteid = FteidBuilder::new()
//!     .teid(0x12345678)
//!     .ipv4("192.168.1.1".parse().unwrap())
//!     .build()
//!     .unwrap();
//!
//! // Create QER using new builder pattern for QoS enforcement
//! let qer = CreateQerBuilder::new(QerId::new(1))
//!     .gate_status(rs_pfcp::ie::gate_status::GateStatus::new(
//!         rs_pfcp::ie::gate_status::GateStatusValue::Open,
//!         rs_pfcp::ie::gate_status::GateStatusValue::Open
//!     ))
//!     .rate_limit(1000000, 2000000) // 1Mbps up, 2Mbps down
//!     .build()
//!     .unwrap();
//!
//! // Create session establishment request
//! # let session_id = 0x1234567890ABCDEF;
//! # let sequence_number = 1;
//! # let node_id = NodeId::new_ipv4("10.0.0.1".parse().unwrap());
//! # let fseid = Fseid::new(0x11111111, None, Some("2001:db8::1".parse().unwrap()));
//! # let pdi = PdiBuilder::uplink_access()
//! #     .f_teid(fteid.clone())
//! #     .build()
//! #     .unwrap();
//! # let create_pdr = CreatePdrBuilder::new(PdrId::new(1))
//! #     .precedence(Precedence::new(100))
//! #     .pdi(pdi)
//! #     .far_id(FarId::new(1))
//! #     .build()
//! #     .unwrap();
//! # let create_far = CreateFar::new(FarId::new(1), ApplyAction::FORW);
//! # let create_qer = CreateQerBuilder::open_gate(QerId::new(1)).build().unwrap();
//! let request = SessionEstablishmentRequestBuilder::new(session_id, sequence_number)
//!     .node_id(node_id.to_ie())
//!     .fseid(Ie::new(IeType::Fseid, fseid.marshal()))
//!     .create_pdrs(vec![create_pdr.to_ie()])
//!     .create_fars(vec![create_far.to_ie()])
//!     .create_qers(vec![create_qer.to_ie()])
//!     .build()
//!     .unwrap();
//!
//! // Serialize to bytes for network transmission
//! let bytes = request.marshal();
//!
//! // Parse received messages
//! let parsed_msg = rs_pfcp::message::parse(&bytes).unwrap();
//!
//! // Convenience methods for common QER patterns
//! # use rs_pfcp::ie::create_qer::CreateQer;
//! let open_qer = CreateQer::open_gate(QerId::new(2));
//! let closed_qer = CreateQer::closed_gate(QerId::new(3));
//! let rate_limited_qer = CreateQer::with_rate_limit(QerId::new(4), 5000000, 10000000);
//! let downlink_only_qer = CreateQer::downlink_only(QerId::new(5));
//! let uplink_only_qer = CreateQer::uplink_only(QerId::new(6));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Module Organization
//!
//! - [`ie`] - Information Elements (IEs) as defined in 3GPP TS 29.244
//! - [`message`] - PFCP message types for session and association management

pub mod ie;
pub mod message;
