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
use crate::message::{IeIter, Message, MsgType, parse_ies, PFCP_HEADER_SIZE};
use crate::types::{SequenceNumber, Seid};  // as needed

/// <MessageTypeName> — 3GPP TS 29.244 Section 7.X.X
#[derive(Debug, Clone, PartialEq)]
pub struct <MessageTypeName> {
    pub sequence: SequenceNumber,
    // pub seid: Seid,  // session messages only
    pub ies: Vec<Ie>,
}

impl <MessageTypeName> {
    pub fn new(sequence: SequenceNumber) -> Self {
        Self { sequence, ies: Vec::new() }
    }
}

impl Message for <MessageTypeName> {
    fn msg_type(&self) -> MsgType { MsgType::<MessageTypeName> }
    fn sequence(&self) -> SequenceNumber { self.sequence }
    fn seid(&self) -> Option<Seid> { None }  // Some(self.seid) for session msgs

    fn marshal(&self) -> Vec<u8> {
        todo!()
    }

    fn unmarshal(data: &[u8]) -> Result<Box<dyn Message>, PfcpError> {
        todo!()
    }

    fn ies(&self, ie_type: IeType) -> IeIter<'_> {
        IeIter::new(self.ies.iter().filter(move |ie| ie.ie_type == ie_type))
    }
}
```

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
