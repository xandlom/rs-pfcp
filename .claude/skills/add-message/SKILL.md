---
name: add-message
description: Scaffold a new PFCP message type following project conventions
argument-hint: "<MessageTypeName> <msg_type_number>"
---

Add a new PFCP message type to the rs-pfcp library. The arguments are the
PascalCase message name and the message type number from 3GPP TS 29.244
Table 7.1.1-1.

Example invocations:
- `/add-message HeartbeatRequest 1`
- `/add-message SessionReportResponse 72`

## Steps

### 1. Read context

Before writing anything, read:
- `src/message/mod.rs` — understand MsgType enum, Message trait, and parse() dispatch
- A similar existing message for reference. For session messages (with SEID):
  `src/message/session_modification_request.rs`. For node messages (no SEID):
  `src/message/heartbeat_request.rs`
- `src/message/display.rs` — to understand how messages plug into display

### 2. Determine message category

**Node-level messages** (no SEID, sequence only):
- HeartbeatRequest/Response, AssociationSetup/Update/Release, NodeReport

**Session-level messages** (with SEID):
- SessionEstablishment/Modification/Deletion and related

### 3. Create the message module

Create `src/message/<snake_case_name>.rs`:

```rust
use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, IeIter, Message, MsgType};
use crate::types::SequenceNumber;  // + Seid for session messages

/// <MessageTypeName> — 3GPP TS 29.244 Section 7.X.X
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct <MessageTypeName> {
    header: Header,
    // One field per mandatory/optional IE (Ie or Option<Ie>), e.g.:
    // recovery_time_stamp: Ie,
    ies: Vec<Ie>, // any remaining/unrecognized IEs
}

impl <MessageTypeName> {
    pub fn new(seq: impl Into<SequenceNumber>, /* mandatory IEs, */ ies: Vec<Ie>) -> Self {
        let mut payload_len = 0usize; // sum .len() of every mandatory/optional Ie + ies
        let mut header = Header::new(MsgType::<MessageTypeName>, /* has_seid */ false, 0, seq);
        header.length = 4 + payload_len;
        Self { header, ies }
    }
}

impl Message for <MessageTypeName> {
    fn marshal(&self) -> Vec<u8> {
        todo!() // header.marshal() + each mandatory/optional Ie.marshal() + ies
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(data)?;
        let mut ies = Vec::new();
        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            offset += ie.len() as usize;
            match ie.ie_type {
                // IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                _ => ies.push(ie),
            }
        }
        // Validate mandatory IEs are present with PfcpError::MissingMandatoryIe
        todo!()
    }

    fn msg_type(&self) -> MsgType {
        MsgType::<MessageTypeName>
    }
    fn seid(&self) -> Option<crate::types::Seid> {
        None // Some(self.seid) for session messages
    }
    fn sequence(&self) -> SequenceNumber {
        self.header.sequence_number
    }
    fn set_sequence(&mut self, seq: SequenceNumber) {
        self.header.sequence_number = seq;
    }
    fn ies(&self, ie_type: IeType) -> IeIter<'_> {
        // IeIter::single(self.recovery_time_stamp.as_ref(), ie_type) for a single
        // mandatory/optional field, IeIter::multiple(&self.create_pdrs, ie_type) for a
        // Vec<Ie> field, or IeIter::generic(&self.ies, ie_type) as a fallback.
        todo!()
    }
    fn all_ies(&self) -> Vec<&Ie> {
        todo!() // collect every mandatory/optional field + self.ies
    }
}
```

`marshal()`, `unmarshal()`, `msg_type()`, `seid()`, `sequence()`, `set_sequence()`,
`ies()`, and `all_ies()` are all required (no default impl) — see the real
`Message` trait in `src/message/mod.rs` for the exact signatures, including
the optional-but-worth-overriding `marshal_into()`/`marshaled_size()`.

If the message is complex, add a builder in the same file following the
pattern in `session_establishment_request.rs`.

### 4. Register in mod.rs

In `src/message/mod.rs`, add in three places:

**a) `MsgType` enum** — in numeric order:
```rust
<MessageTypeName> = <number>,
```

**b) Module declaration and re-export**:
```rust
pub mod <snake_case_name>;
pub use <snake_case_name>::<MessageTypeName>;
```

**c) `parse()` dispatch match arm**:
```rust
MsgType::<MessageTypeName> => <MessageTypeName>::unmarshal(data)?,
```

### 5. Add display support

In `src/message/display.rs`, add a match arm for the new message type so it
renders correctly in YAML/JSON output via `pcap-reader`.

### 6. Write tests

In `src/message/<snake_case_name>.rs`, add a `#[cfg(test)]` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = <MessageTypeName>::new(SequenceNumber::new(1));
        let bytes = original.marshal();
        let parsed = <MessageTypeName>::unmarshal(&bytes).unwrap();
        // compare key fields
    }
}
```

Also add an integration test to `tests/messages.rs` following existing patterns.

### 7. Verify

```bash
cargo test message::<snake_case_name>
cargo test --test messages
cargo clippy --all-targets --all-features -- -D warnings
```

### 8. Update docs/reference/messages.md

Mark the message as implemented in `docs/reference/messages.md`.

### 9. Commit

```
feat(message): add <MessageTypeName> (type <number>)
```
