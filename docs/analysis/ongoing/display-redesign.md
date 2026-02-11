# Display Module Redesign

**Status:** Proposed
**File:** `src/message/display.rs`
**Current size:** 2,146 lines
**Target size:** ~500-600 lines (~70% reduction)

## Problem Statement

The current `display.rs` has severe structural issues:

1. **Massive duplication** — Every IE conversion function exists twice (YAML + JSON).
   8 pairs of `*_to_structured_data()` / `*_to_json_data()` functions.
2. **`Box<dyn Message>` impl is a 170-line copy** of the blanket `impl<T: Message>`.
3. **Hardcoded IE type list** — `get_common_ie_types()` returns 24 types manually.
   Adding a new IE means remembering to update this list.
4. **IE order is lost** — The current design iterates a hardcoded list of IE types,
   not the actual message IEs. Output order reflects the list, not the wire format.
5. **Multi-instance match arm** duplicated 4 times across YAML/JSON and generic/boxed impls.

## Design Principles

1. **Preserve IE order** — Output MUST reflect the binary message order.
2. **Single intermediate representation** — Build once, serialize to any format.
3. **No duplication** — Each IE display logic written exactly once.
4. **Self-describing IEs** — Each IE carries its type, no map key required.
5. **Easy extension** — Adding display for a new IE = one match arm.

## Architecture

```
┌───────────────────────────────────────────────────┐
│  Layer 3: Public API  (thin wrappers)             │
│  MessageDisplay::to_yaml / to_json / to_json_pretty│
│  Calls serde on the Value from Layer 2            │
└──────────────────────┬────────────────────────────┘
                       │
┌──────────────────────▼────────────────────────────┐
│  Layer 2: Message → Value  (single implementation)│
│  message_to_value(&dyn Message) → JsonValue       │
│  Uses all_ies() — preserves wire order            │
└──────────────────────┬────────────────────────────┘
                       │  calls for each IE
┌──────────────────────▼────────────────────────────┐
│  Layer 1: IE → Value  (single dispatch)           │
│  ie_to_value(&Ie) → JsonValue                     │
│  rich_display() per IE type, hex fallback         │
└───────────────────────────────────────────────────┘
```

### Key Decision: `serde_json::Value` as the Only Intermediate Type

`serde_json::Value` implements `serde::Serialize`, so:

- `serde_json::to_string(&value)` → compact JSON
- `serde_json::to_string_pretty(&value)` → pretty JSON
- `serde_yaml_ng::to_string(&value)` → YAML

No `YamlValue` anywhere. No custom enum. No duplication.

### Key Decision: Array-Based IE Output (Order Preservation)

IEs are output as an **ordered array**, not a map keyed by type name.
This preserves binary message order and avoids the grouping/dedup complexity.

**Current format (map-based, loses order):**
```yaml
information_elements:
  nodeid:
    type: NodeId
    length: 5
    address: 192.168.1.1
  createpdr:
    - type: CreatePdr
      pdr_id: 1
    - type: CreatePdr
      pdr_id: 2
  createfar:
    type: CreateFar
    far_id: 1
```

**New format (array-based, preserves wire order):**
```yaml
information_elements:
  - type: NodeId
    length: 5
    address: 192.168.1.1
  - type: CreatePdr
    length: 24
    pdr_id: 1
  - type: CreateFar
    length: 16
    far_id: 1
  - type: CreatePdr
    length: 24
    pdr_id: 2
```

Advantages:
- Order matches the binary message exactly
- No special-casing for single vs multiple IEs of the same type
- Each IE is self-describing (has its own `type` field)
- Simpler code — no grouping logic

## Detailed Design

### Layer 1: IE Display

```rust
use serde_json::{json, Value, Map};

/// Convert a single IE to a JSON value with type-specific rich display.
fn ie_to_value(ie: &Ie) -> Value {
    let mut obj = Map::new();
    obj.insert("type".into(), json!(format!("{:?}", ie.ie_type)));
    obj.insert("length".into(), json!(ie.len()));

    match rich_display(ie) {
        Some(fields) => obj.extend(fields),
        None => add_fallback_payload(&mut obj, &ie.payload),
    }

    Value::Object(obj)
}

/// Try to produce rich display fields for known IE types.
/// Returns None for unsupported types (falls back to hex).
fn rich_display(ie: &Ie) -> Option<Map<String, Value>> {
    match ie.ie_type {
        IeType::NodeId => display_node_id(&ie.payload),
        IeType::Cause => display_cause(&ie.payload),
        IeType::RecoveryTimeStamp => display_recovery_timestamp(&ie.payload),
        IeType::Fseid => display_fseid(&ie.payload),
        IeType::CreatePdr => display_create_pdr(&ie.payload),
        IeType::CreatedPdr => display_created_pdr(&ie.payload),
        IeType::CreateFar => display_create_far(&ie.payload),
        IeType::ReportType => display_report_type(&ie.payload),
        IeType::UsageReportWithinSessionReportRequest => display_usage_report(&ie.payload),
        IeType::EthernetPduSessionInformation => display_ethernet_pdu_info(&ie.payload),
        IeType::EthernetContextInformation => display_ethernet_context(&ie.payload),
        IeType::EthernetInactivityTimer => display_ethernet_inactivity_timer(&ie.payload),
        _ => None,
    }
}

/// Fallback: hex dump for small payloads, size for large ones.
fn add_fallback_payload(obj: &mut Map<String, Value>, payload: &[u8]) {
    if payload.len() <= 32 {
        let hex = payload.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
        obj.insert("payload_hex".into(), json!(hex));
    } else {
        obj.insert("payload_size".into(), json!(payload.len()));
    }
}
```

Each `display_*` function is a small standalone function:

```rust
fn display_node_id(payload: &[u8]) -> Option<Map<String, Value>> {
    let node_id = NodeId::unmarshal(payload).ok()?;
    let (node_type, address) = match &node_id {
        NodeId::IPv4(ip) => ("IPv4", ip.to_string()),
        NodeId::IPv6(ip) => ("IPv6", ip.to_string()),
        NodeId::FQDN(fqdn) => ("FQDN", fqdn.clone()),
    };
    Some(Map::from_iter([
        ("node_type".into(), json!(node_type)),
        ("address".into(), json!(address)),
    ]))
}

fn display_cause(payload: &[u8]) -> Option<Map<String, Value>> {
    let cause = Cause::unmarshal(payload).ok()?;
    Some(Map::from_iter([
        ("cause_value".into(), json!(cause.value as u8)),
        ("cause_name".into(), json!(format!("{:?}", cause.value))),
    ]))
}
// ... one function per supported IE type
```

### Layer 2: Message Display

```rust
/// Convert any message to a JSON value. Single source of truth.
/// IE order matches the binary message.
fn message_to_value(msg: &dyn Message) -> Value {
    let mut map = Map::new();

    // Metadata
    map.insert("message_type".into(), json!(msg.msg_name()));
    map.insert("sequence".into(), json!(*msg.sequence()));
    map.insert("version".into(), json!(msg.version()));

    if let Some(seid) = msg.seid() {
        map.insert("seid".into(), json!(*seid));
    }

    // IEs — preserving wire order via all_ies()
    let all = msg.all_ies();
    if !all.is_empty() {
        let ie_array: Vec<Value> = all.iter().map(|ie| ie_to_value(ie)).collect();
        map.insert("information_elements".into(), Value::Array(ie_array));
    }

    Value::Object(map)
}
```

### Layer 3: Public Trait

```rust
pub trait MessageDisplay {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error>;
    fn to_json(&self) -> Result<String, serde_json::Error>;
    fn to_json_pretty(&self) -> Result<String, serde_json::Error>;
}

impl<T: Message> MessageDisplay for T {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(&message_to_value(self))
    }
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&message_to_value(self))
    }
    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&message_to_value(self))
    }
}

impl MessageDisplay for Box<dyn Message> {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(&message_to_value(self.as_ref()))
    }
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&message_to_value(self.as_ref()))
    }
    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&message_to_value(self.as_ref()))
    }
}
```

### What Gets Removed

| Current code | Replacement |
|---|---|
| `ie_to_structured_data()` (153 lines) | `ie_to_value()` (~20 lines) + `rich_display()` dispatch |
| `ie_to_json_data()` (93 lines) | eliminated (same as above) |
| `node_id_to_structured_data` + `node_id_to_json_data` | `display_node_id` (1 function) |
| `cause_to_structured_data` + `cause_to_json_data` | `display_cause` (1 function) |
| `usage_report_to_structured_data` + `_json_data` | `display_usage_report` (1 function) |
| `create_far_to_structured_data` + `_json_data` | `display_create_far` (1 function) |
| `recovery_timestamp_to_structured_data` + `_json_data` | `display_recovery_timestamp` (1 function) |
| `fseid_to_structured_data` + `_json_data` | `display_fseid` (1 function) |
| `create_pdr_to_structured_data` + `_json_data` | `display_create_pdr` (1 function) |
| `created_pdr_to_structured_data` + `_json_data` | `display_created_pdr` (1 function) |
| `to_structured_data` on `impl<T: Message>` (78 lines) | `message_to_value` (1 function, ~20 lines) |
| `to_json_data` on `impl<T: Message>` (74 lines) | eliminated |
| `impl MessageDisplay for Box<dyn Message>` (170 lines) | 9-line delegation |
| `get_common_ie_types()` (28 lines) | eliminated (`all_ies()` used directly) |
| Multi-instance match arm (13 variants, ×4) | eliminated (array format, no grouping) |

## Public API Changes

### Trait API: No Change

```rust
pub trait MessageDisplay {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error>;
    fn to_json(&self) -> Result<String, serde_json::Error>;
    fn to_json_pretty(&self) -> Result<String, serde_json::Error>;
}
```

### Removed from Public API

`to_structured_data()` and `to_json_data()` are currently public trait methods.
They are only used internally (by `to_yaml`/`to_json`/`to_json_pretty`) and in tests.
They are replaced by the private `message_to_value()` function.

If external callers need structured access, they can use:
```rust
let value: serde_json::Value = serde_json::from_str(&msg.to_json()?)?;
```

### Output Format: Breaking Change

The `information_elements` field changes from a **map** (keyed by lowercase IE type name)
to an **array** (ordered list matching wire format). See the format comparison above.

**Affected consumers:**
- `examples/pcap-reader/main.rs` — prints YAML/JSON to stdout (no parsing, safe)
- `examples/test_real_messages.rs` — prints YAML to stdout (safe)
- `examples/debug_parser.rs` — prints YAML to stdout (safe)
- `examples/session-server/main.rs` — prints YAML/JSON to stdout (safe)

All consumers just print the formatted string. None parse the output structure.
This is a safe breaking change.

## Implementation Steps

### Step 1: Write the new module

Rewrite `src/message/display.rs` from scratch with the 3-layer architecture.
Port all existing `display_*` functions to the new single-value pattern.

### Step 2: Update tests

Rewrite the test module to match the new array-based output format.
Tests should verify:
- Round-trip: message → JSON → parse → check structure
- IE order preservation: IEs appear in output in binary message order
- Rich display: known IE types produce structured fields
- Fallback: unknown IE types produce hex/size payload
- Format equivalence: YAML and JSON contain the same data

### Step 3: Verify examples compile and run

```bash
cargo build --examples
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Step 4: Manual verification

Run pcap-reader and session-server with sample data to verify
output is readable and correct.

## Testing Strategy

### Order Preservation Test

```rust
#[test]
fn test_ie_order_matches_wire_format() {
    // Build a message with IEs in a specific order
    let msg = build_message_with_known_ie_order();
    let json_str = msg.to_json().unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    let ies = parsed["information_elements"].as_array().unwrap();
    // Verify IEs appear in the same order as in the message
    assert_eq!(ies[0]["type"], "NodeId");
    assert_eq!(ies[1]["type"], "Fseid");
    assert_eq!(ies[2]["type"], "CreatePdr");
    assert_eq!(ies[3]["type"], "CreateFar");
}
```

### Rich Display Tests

```rust
#[test]
fn test_node_id_rich_display() {
    let ie = Ie::new(IeType::NodeId, NodeId::new_ipv4(addr).marshal().to_vec());
    let value = ie_to_value(&ie);
    assert_eq!(value["type"], "NodeId");
    assert_eq!(value["node_type"], "IPv4");
    assert_eq!(value["address"], "192.168.1.1");
}
```

### Fallback Display Test

```rust
#[test]
fn test_unknown_ie_hex_fallback() {
    let ie = Ie::new(IeType::SomeUnknownType, vec![0xDE, 0xAD]);
    let value = ie_to_value(&ie);
    assert_eq!(value["payload_hex"], "de ad");
}
```

## Estimated Line Counts

| Section | Current | New |
|---|---|---|
| Public trait + impls | ~220 lines | ~35 lines |
| `message_to_value` | ~300 lines (×2 for yaml/json, ×2 for generic/boxed) | ~25 lines |
| IE dispatch + rich display | ~1,100 lines (×2 for yaml/json) | ~300 lines |
| Helpers (`get_common_ie_types`, etc.) | ~30 lines | 0 (eliminated) |
| Tests | ~500 lines | ~250 lines |
| **Total** | **~2,150 lines** | **~610 lines** |
