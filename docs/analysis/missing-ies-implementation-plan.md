# Missing IEs Implementation Plan

## Overview

**Current state:** 157 IE modules implemented out of ~336 IeType enum variants (excluding `Unknown`)
**Missing:** 179 IE implementations needed
**Goal:** Full 3GPP TS 29.244 Release 18 IE coverage

This plan organizes the 179 missing IEs into 10 implementation phases, grouped by functional area and complexity. Each phase includes the IE encoding pattern, 3GPP spec reference, and a code example.

---

## IE Encoding Patterns (Reference)

All missing IEs fall into one of these patterns already used in the codebase:

| Pattern | Description | Existing Example | Count |
|---------|-------------|-----------------|-------|
| **Simple u32** | Fixed 4-byte unsigned integer | `precedence.rs` | ~25 |
| **Simple u16** | Fixed 2-byte unsigned integer | `pdr_id.rs` | ~8 |
| **Simple u8** | Fixed 1-byte value or flags | `measurement_method.rs` | ~30 |
| **Flags byte** | 1-byte bitfield with named flags | `pfcpsm_req_flags.rs` | ~15 |
| **Volume-style** | Flags byte + conditional u64 fields | `volume_quota.rs` | ~5 |
| **Octet String** | Variable-length byte string | `forwarding_policy.rs` | ~10 |
| **IP Address** | IPv4/IPv6 address with flags | `alternative_smf_ip_address.rs` | ~8 |
| **Grouped** | Contains child IEs (TLV-encoded) | `create_pdr.rs` | ~30 |
| **Timestamp** | 4-byte NTP timestamp | `start_time.rs` | ~3 |
| **UUID** | 16-byte UUID v4 | (new) | 1 |
| **Multiplier-style** | Struct with specific fixed fields | `multiplier.rs` | ~3 |

---

## Phase 1: Simple Value IEs (Priority: HIGH)

**Effort:** Low per IE | **Total IEs:** 25
**Pattern:** Fixed-length integer values (u8/u16/u32/u64)
**3GPP Spec:** Section 8.2.x for each

These are the simplest to implement — each is a fixed-size integer with marshal/unmarshal.

### IEs in this phase:

| IE Name | Type | Encoding | 3GPP Section |
|---------|------|----------|-------------|
| MeasurementPeriod | 64 | u32 (seconds) | 8.2.42 |
| AggregatedUrrId | 120 | u32 | 8.2.85 |
| SubsequentTimeQuota | 122 | u32 (seconds) | 8.2.87 |
| EventQuota | 148 | u32 | 8.2.112 |
| EventThreshold | 149 | u32 | 8.2.113 |
| SubsequentEventQuota | 150 | u32 | 8.2.106 |
| SubsequentEventThreshold | 151 | u32 | 8.2.107 |
| EventTimeStamp | 156 | u32 (NTP timestamp) | 8.2.114 |
| MarId | 170 | u16 | 8.2.123 |
| Weight | 173 | u8 (0-100) | 8.2.126 |
| QuotaValidityTime | 181 | u32 (seconds) | 8.2.132 |
| NumberOfReports | 182 | u16 (excl. 0) | 8.2.133 |
| DsttPortNumber | 196 | u16 | 8.2.149 |
| NwttPortNumber | 197 | u16 | 8.2.150 |
| TsnTimeDomainNumber | 206 | u8 | 8.2.157 |
| TimeOffsetThreshold | 207 | u64 (nanoseconds) | 8.2.158 |
| CumulativeRateRatioThreshold | 208 | u32 | 8.2.159 |
| SrrId | 215 | u8 | 8.2.166 |
| AveragePacketDelay | 234 | u32 (microseconds) | 8.2.181 |
| MinimumPacketDelay | 235 | u32 (microseconds) | 8.2.182 |
| MaximumPacketDelay | 236 | u32 (microseconds) | 8.2.183 |
| MinimumWaitTime | 246 | u32 (seconds) | 8.2.192 |
| DlDataPacketsSize | 250 | u16 | 8.2.195 |
| ValidityTimer | 269 | u16 (seconds) | 8.2.207 |
| AreaSessionId | 314 | u16 | 8.2.232 |

### Example implementation — `MeasurementPeriod` (IE Type 64):

```rust
// src/ie/measurement_period.rs
//! Measurement Period IE - 3GPP TS 29.244 Section 8.2.42
//!
//! Indicates the measurement period in seconds for volume/duration measurements.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Measurement Period in seconds.
///
/// Per 3GPP TS 29.244 Section 8.2.42, encoded as Unsigned32.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
///
/// let period = MeasurementPeriod::new(3600); // 1 hour
/// let bytes = period.marshal();
/// let parsed = MeasurementPeriod::unmarshal(&bytes).unwrap();
/// assert_eq!(period, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeasurementPeriod {
    /// Measurement period in seconds
    pub seconds: u32,
}

impl MeasurementPeriod {
    pub fn new(seconds: u32) -> Self {
        Self { seconds }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.seconds.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Measurement Period",
                IeType::MeasurementPeriod,
                4,
                data.len(),
            ));
        }
        let seconds = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(Self { seconds })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::from_marshal(IeType::MeasurementPeriod, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MeasurementPeriod::new(3600);
        let bytes = original.marshal();
        let parsed = MeasurementPeriod::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        let result = MeasurementPeriod::unmarshal(&[0x00, 0x01]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie() {
        let period = MeasurementPeriod::new(60);
        let ie = period.to_ie();
        assert_eq!(ie.ie_type, IeType::MeasurementPeriod);
    }

    #[test]
    fn test_boundary_values() {
        for val in [0u32, 1, u32::MAX] {
            let original = MeasurementPeriod::new(val);
            let bytes = original.marshal();
            let parsed = MeasurementPeriod::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed);
        }
    }
}
```

### Steps for each simple value IE:
1. Create `src/ie/<ie_name>.rs` following the pattern above
2. Change the type to u16/u8/u64 as needed per spec encoding
3. Add `pub mod <ie_name>;` to `src/ie/mod.rs`
4. Write round-trip, error, and boundary tests

---

## Phase 2: Flags / Single-Byte IEs (Priority: HIGH)

**Effort:** Low per IE | **Total IEs:** 18
**Pattern:** 1-byte bitfield with named boolean flags
**3GPP Spec:** Section 8.2.x for each

### IEs in this phase:

| IE Name | Type | Flags | 3GPP Section |
|---------|------|-------|-------------|
| OciFlags | 110 | AOCI | 8.2.76 |
| SteeringFunctionality | 171 | 4-bit enum (ATSSS-LL/MPTCP/MPQUIC) | 8.2.124 |
| SteeringMode | 172 | 4-bit enum (Active-Standby/SmallestDelay/LoadBalancing/Priority/Redundant) | 8.2.125 |
| Priority | 174 | 4-bit value (Active=0/Standby=1/High=2/Low=3) | 8.2.127 |
| PacketReplicationAndDetectionCarryOnInformation | 179 | PRIUEAI/PRIN19I/PRIN6I/DCARONI | 8.2.130 |
| PfcpseReqFlags | 186 | RESTI | 8.2.137 |
| RequestedClockDriftInformation | 204 | RRCR/RRTO | 8.2.155 |
| RequestedAccessAvailabilityInformation | 217 | RRCA | 8.2.168 |
| AccessAvailabilityInformation | 219 | flags per access type | 8.2.170 |
| QosReportTrigger | 237 | PRR/PIR/PER | 8.2.184 |
| GtpuPathInterfaceType | 241 | N9/N3 | 8.2.188 |
| RequestedQosMonitoring | 243 | RP/UL/DL | 8.2.190 |
| ReportingFrequency | 244 | EVETT/PERIO/SESRL | 8.2.191 |
| IpVersion | 258 | V4/V6 | 8.2.177 |
| PfcpasReqFlags | 259 | UUPSI | 8.2.178 |
| DataStatus | 260 | DROP/BUFF | 8.2.179 |
| MptcpApplicableIndication | 265 | MAI | 8.2.203 |
| PfcpsdrspFlags | 318 | flags | 8.2.236 |

### Example implementation — `OciFlags` (IE Type 110):

```rust
// src/ie/oci_flags.rs
//! OCI Flags IE - 3GPP TS 29.244 Section 8.2.76
//!
//! Overload Control Information Flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// OCI Flags
///
/// Per 3GPP TS 29.244 Section 8.2.76.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::oci_flags::OciFlags;
///
/// let flags = OciFlags::new(true);
/// assert!(flags.aoci());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OciFlags {
    /// Associate OCI with Node ID
    aoci: bool,
}

impl OciFlags {
    pub fn new(aoci: bool) -> Self {
        Self { aoci }
    }

    /// Returns true if AOCI (Associate OCI with Node ID) is set.
    pub fn aoci(&self) -> bool {
        self.aoci
    }

    pub fn marshal(&self) -> [u8; 1] {
        let mut b: u8 = 0;
        if self.aoci {
            b |= 0x01;
        }
        [b]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "OCI Flags",
                IeType::OciFlags,
                1,
                0,
            ));
        }
        Ok(Self {
            aoci: (data[0] & 0x01) != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::from_marshal(IeType::OciFlags, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        for aoci in [true, false] {
            let original = OciFlags::new(aoci);
            let bytes = original.marshal();
            let parsed = OciFlags::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            OciFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let flags = OciFlags::new(true);
        let ie = flags.to_ie();
        assert_eq!(ie.ie_type, IeType::OciFlags);
    }
}
```

### Example — `SteeringFunctionality` (IE Type 171, enum-style):

```rust
// src/ie/steering_functionality.rs
//! Steering Functionality IE - 3GPP TS 29.244 Section 8.2.124

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Steering Functionality values.
///
/// Per 3GPP TS 29.244 Section 8.2.124.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SteeringFunctionalityValue {
    AtsssLl = 0,
    Mptcp = 1,
    Mpquic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SteeringFunctionality {
    pub value: SteeringFunctionalityValue,
}

impl SteeringFunctionality {
    pub fn new(value: SteeringFunctionalityValue) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Steering Functionality",
                IeType::SteeringFunctionality,
                1,
                0,
            ));
        }
        let value = match data[0] & 0x0F {
            0 => SteeringFunctionalityValue::AtsssLl,
            1 => SteeringFunctionalityValue::Mptcp,
            2 => SteeringFunctionalityValue::Mpquic,
            v => return Err(PfcpError::invalid_value(
                "Steering Functionality",
                v.to_string(),
                "must be 0 (ATSSS-LL), 1 (MPTCP), or 2 (MPQUIC)",
            )),
        };
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::from_marshal(IeType::SteeringFunctionality, self.marshal())
    }
}
```

---

## Phase 3: Volume/Threshold IEs with Flags (Priority: HIGH)

**Effort:** Medium per IE | **Total IEs:** 5
**Pattern:** Flags byte + conditional u64/u32 fields (same as `VolumeQuota`)

### IEs in this phase:

| IE Name | Type | Pattern | 3GPP Section |
|---------|------|---------|-------------|
| DroppedDlTrafficThreshold | 72 | flags (DLPA/DLBY) + optional u64 packets + u64 bytes | 8.2.49 |
| SubsequentVolumeQuota | 121 | flags (TOVOL/ULVOL/DLVOL) + optional u64 fields | 8.2.86 |
| TimeQuotaMechanism | 115 | 1-byte BTIT enum + u32 base time interval | 8.2.81 |
| PacketDelayThresholds | 245 | flags (DL/UL/RP) + optional u32 delay values | 8.2.193 |
| TimeOffsetMeasurement | 209 | i64 signed nanoseconds | 8.2.160 |

### Example — `DroppedDlTrafficThreshold` (IE Type 72):

```rust
// src/ie/dropped_dl_traffic_threshold.rs
//! Dropped DL Traffic Threshold IE - 3GPP TS 29.244 Section 8.2.49

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Dropped DL Traffic Threshold
///
/// Per 3GPP TS 29.244 Section 8.2.49.
/// Flags: Bit 1 = DLPA (DL Packets), Bit 2 = DLBY (DL Bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DroppedDlTrafficThreshold {
    pub flags: u8,
    pub downlink_packets: Option<u64>,
    pub downlink_bytes: Option<u64>,
}

impl DroppedDlTrafficThreshold {
    pub fn new(downlink_packets: Option<u64>, downlink_bytes: Option<u64>) -> Self {
        let mut flags = 0u8;
        if downlink_packets.is_some() {
            flags |= 0x01; // DLPA
        }
        if downlink_bytes.is_some() {
            flags |= 0x02; // DLBY
        }
        Self { flags, downlink_packets, downlink_bytes }
    }

    pub fn has_downlink_packets(&self) -> bool { (self.flags & 0x01) != 0 }
    pub fn has_downlink_bytes(&self) -> bool { (self.flags & 0x02) != 0 }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 16);
        buf.push(self.flags);
        if let Some(pkts) = self.downlink_packets {
            buf.extend_from_slice(&pkts.to_be_bytes());
        }
        if let Some(bytes) = self.downlink_bytes {
            buf.extend_from_slice(&bytes.to_be_bytes());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Dropped DL Traffic Threshold",
                IeType::DroppedDlTrafficThreshold,
                1,
                0,
            ));
        }
        let flags = data[0];
        let mut offset = 1;
        let mut result = Self { flags, downlink_packets: None, downlink_bytes: None };

        if result.has_downlink_packets() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Dropped DL Traffic Threshold (packets)",
                    IeType::DroppedDlTrafficThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            result.downlink_packets = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
            offset += 8;
        }
        if result.has_downlink_bytes() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Dropped DL Traffic Threshold (bytes)",
                    IeType::DroppedDlTrafficThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            result.downlink_bytes = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
        }
        Ok(result)
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DroppedDlTrafficThreshold, self.marshal())
    }
}
```

---

## Phase 4: Octet String / Variable-Length IEs (Priority: MEDIUM)

**Effort:** Low-Medium per IE | **Total IEs:** 12
**Pattern:** Variable-length byte/string data

### IEs in this phase:

| IE Name | Type | Encoding | 3GPP Section |
|---------|------|----------|-------------|
| FramedRoute | 153 | Octet String (RFC 2865 format) | 8.2.109 |
| FramedRouting | 154 | u32 (RFC 2865 AVP value) | 8.2.110 |
| FramedIpv6Route | 155 | Octet String (RFC 3162 format) | 8.2.111 |
| UeIpAddressPoolIdentity | 177 | u16 length + Octet String | 8.2.128 |
| DataNetworkAccessIdentifier | 232 | Octet String (NAI format) | 8.2.178 |
| PortManagementInformationContainer | 202 | Octet String (opaque) | 8.2.153 |
| BridgeManagementInformationContainer | 266 | Octet String (opaque) | 8.2.204 |
| TunnelPassword | 313 | Octet String | 8.2.231 |
| Metadata | 322 | Octet String | 8.2.240 |
| ProtocolDescription | 334 | Octet String | 8.2.252 |
| Uri | 352 | Octet String (URI format) | 8.2.270 |
| TlContainer | 336 | Octet String (opaque TL-Container) | 8.2.254 |

### Example — `FramedRoute` (IE Type 153):

```rust
// src/ie/framed_route.rs
//! Framed-Route IE - 3GPP TS 29.244 Section 8.2.109
//!
//! Contains a Framed-Route AVP value as defined in RFC 2865.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Framed-Route
///
/// Per 3GPP TS 29.244 Section 8.2.109.
/// Encoded as an Octet String containing the Framed-Route AVP value per RFC 2865.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::framed_route::FramedRoute;
///
/// let route = FramedRoute::new(b"10.0.0.0/8 192.168.1.1 1".to_vec());
/// let bytes = route.marshal();
/// let parsed = FramedRoute::unmarshal(&bytes).unwrap();
/// assert_eq!(route, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FramedRoute {
    pub value: Vec<u8>,
}

impl FramedRoute {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    /// Create from a string route description.
    pub fn from_str(route: &str) -> Self {
        Self { value: route.as_bytes().to_vec() }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        // Framed-Route can be variable length but must not be empty
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Framed-Route",
                IeType::FramedRoute,
                1,
                0,
            ));
        }
        Ok(Self { value: data.to_vec() })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FramedRoute, self.marshal())
    }
}
```

---

## Phase 5: IP Address IEs (Priority: MEDIUM)

**Effort:** Medium per IE | **Total IEs:** 8
**Pattern:** Flags byte + conditional IPv4 (4 bytes) and/or IPv6 (16 bytes)

### IEs in this phase:

| IE Name | Type | Encoding | 3GPP Section |
|---------|------|----------|-------------|
| CpPfcpEntityIpAddress | 185 | flags (V4/V6) + addresses | 8.2.136 |
| IpMulticastAddress | 191 | flags + IPv4/IPv6 source/any | 8.2.143 |
| MptcpAddressInformation | 228 | type + port + addresses | 8.2.175 |
| UeLinkSpecificIpAddress | 229 | flags + addresses | 8.2.176 |
| PmfAddressInformation | 230 | flags + port + addresses | 8.2.177 |
| TsnBridgeId | 198 | MAC address (6 bytes) | 8.2.151 |
| MpquicAddressInformation | 332 | flags + port + addresses | 8.2.250 |
| MappedN6IpAddress | 350 | flags + IPv4/IPv6 | 8.2.268 |

### Example — `CpPfcpEntityIpAddress` (IE Type 185):

```rust
// src/ie/cp_pfcp_entity_ip_address.rs
//! CP PFCP Entity IP Address IE - 3GPP TS 29.244 Section 8.2.136

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// CP PFCP Entity IP Address
///
/// Per 3GPP TS 29.244 Section 8.2.136.
/// Bit 1 = V4, Bit 2 = V6
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CpPfcpEntityIpAddress {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl CpPfcpEntityIpAddress {
    pub fn from_ipv4(addr: Ipv4Addr) -> Self {
        Self { ipv4: Some(addr), ipv6: None }
    }

    pub fn from_ipv6(addr: Ipv6Addr) -> Self {
        Self { ipv4: None, ipv6: Some(addr) }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut flags = 0u8;
        if self.ipv4.is_some() { flags |= 0x01; }
        if self.ipv6.is_some() { flags |= 0x02; }
        buf.push(flags);
        if let Some(v4) = self.ipv4 {
            buf.extend_from_slice(&v4.octets());
        }
        if let Some(v6) = self.ipv6 {
            buf.extend_from_slice(&v6.octets());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "CP PFCP Entity IP Address",
                IeType::CpPfcpEntityIpAddress,
                1,
                0,
            ));
        }
        let flags = data[0];
        let mut offset = 1;
        let ipv4 = if (flags & 0x01) != 0 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "CP PFCP Entity IP Address (IPv4)",
                    IeType::CpPfcpEntityIpAddress,
                    offset + 4,
                    data.len(),
                ));
            }
            let addr = Ipv4Addr::new(data[offset], data[offset+1], data[offset+2], data[offset+3]);
            offset += 4;
            Some(addr)
        } else {
            None
        };
        let ipv6 = if (flags & 0x02) != 0 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "CP PFCP Entity IP Address (IPv6)",
                    IeType::CpPfcpEntityIpAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let bytes: [u8; 16] = data[offset..offset+16].try_into().unwrap();
            Some(Ipv6Addr::from(bytes))
        } else {
            None
        };
        Ok(Self { ipv4, ipv6 })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CpPfcpEntityIpAddress, self.marshal())
    }
}
```

---

## Phase 6: Structured IEs with Multiple Fields (Priority: MEDIUM)

**Effort:** Medium per IE | **Total IEs:** 10
**Pattern:** Multi-field structs with specific encoding

### IEs in this phase:

| IE Name | Type | Encoding | 3GPP Section |
|---------|------|----------|-------------|
| TimeQuotaMechanism | 115 | 2-bit BTIT enum + u32 base interval | 8.2.81 |
| UserPlaneIpResourceInformation | 116 | complex: flags + TEID range + addresses + NI | 8.2.82 |
| NfInstanceId | 253 | UUID v4 (16 bytes) | 8.2.175 |
| CumulativeRateRatioMeasurement | 210 | u32 numerator + u32 denominator | 8.2.161 |
| QosMonitoringMeasurement | 248 | flags + u32 DL/UL/RP delays | 8.2.194 |
| PacketRateStatusReport | 252 | grouped: PacketRateStatus + QER ID | 8.2.196 (grouped) |
| NumberOfUeIpAddresses | 268 | flags + u32 IPv4 count + u32 IPv6 count | 8.2.206 |
| AdditionalMonitoringTime | 147 | grouped: MonitoringTime + thresholds | 8.2.x (grouped) |
| DlBufferingSuggestedPacketCount | 48 | u8 or u16 (variable) | 8.2.27 |
| PfcpAssociationReleaseRequest | 111 | flags (SARR/URSS) | 8.2.77 |

### Example — `NfInstanceId` (IE Type 253, UUID):

```rust
// src/ie/nf_instance_id.rs
//! NF Instance ID IE - 3GPP TS 29.244 Section 8.2.175
//!
//! UUID version 4 identifying an NF instance (per RFC 4122).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// NF Instance ID (UUID v4)
///
/// Per 3GPP TS 29.244 Section 8.2.175. Encoded as 16-byte UUID per RFC 4122.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::nf_instance_id::NfInstanceId;
///
/// let uuid_bytes = [
///     0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4,
///     0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00,
/// ];
/// let nf_id = NfInstanceId::new(uuid_bytes);
/// let bytes = nf_id.marshal();
/// let parsed = NfInstanceId::unmarshal(&bytes).unwrap();
/// assert_eq!(nf_id, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NfInstanceId {
    pub uuid: [u8; 16],
}

impl NfInstanceId {
    pub fn new(uuid: [u8; 16]) -> Self {
        Self { uuid }
    }

    pub fn marshal(&self) -> [u8; 16] {
        self.uuid
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 16 {
            return Err(PfcpError::invalid_length(
                "NF Instance ID",
                IeType::NfInstanceId,
                16,
                data.len(),
            ));
        }
        let uuid: [u8; 16] = data[..16].try_into().unwrap();
        Ok(Self { uuid })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::from_marshal(IeType::NfInstanceId, self.marshal())
    }
}
```

---

## Phase 7: MAR (Multi-Access Rules) IEs (Priority: MEDIUM)

**Effort:** High (grouped IEs with builders) | **Total IEs:** 10
**Pattern:** Grouped IEs containing child IEs + simple value IEs
**3GPP Spec:** Sections 7.5.2.8, 7.5.4.8, 8.2.123-127

### IEs in this phase:

| IE Name | Type | Pattern | 3GPP Section |
|---------|------|---------|-------------|
| CreateMar | 165 | Grouped: MAR ID + Steering Func + Steering Mode + 3GPP/Non-3GPP forwarding | 7.5.2.8 |
| TgppAccessForwardingActionInformation | 166 | Grouped: FAR ID + Weight + Priority + URR ID | 7.5.2.8-2 |
| NonTgppAccessForwardingActionInformation | 167 | Grouped: FAR ID + Weight + Priority + URR ID | 7.5.2.8-3 |
| RemoveMar | 168 | Grouped: MAR ID | 7.5.4.8 |
| UpdateMar | 169 | Grouped: MAR ID + updates | 7.5.4.8 |
| UpdateTgppAccessForwardingActionInformation | 175 | Grouped: FAR ID + Weight + Priority + URR ID | 7.5.4.8-2 |
| UpdateNonTgppAccessForwardingActionInformation | 176 | Grouped: FAR ID + Weight + Priority + URR ID | 7.5.4.8-3 |

**Dependencies:** Phase 1 (MarId) and Phase 2 (SteeringFunctionality, SteeringMode, Weight, Priority) must be completed first.

### Example — `CreateMar` (IE Type 165):

```rust
// src/ie/create_mar.rs
//! Create MAR (Multi-Access Rule) IE - 3GPP TS 29.244 Section 7.5.2.8

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Create MAR grouped IE.
///
/// Contains: MAR ID (M), Steering Functionality (M), Steering Mode (M),
/// 3GPP Access Forwarding Action Info (C), Non-3GPP Access Forwarding Action Info (C)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMar {
    ies: Vec<Ie>,
}

impl CreateMar {
    pub fn new(ies: Vec<Ie>) -> Self {
        Self { ies }
    }

    pub fn ies(&self) -> &[Ie] {
        &self.ies
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for ie in &self.ies {
            ie.marshal_into(&mut buf);
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let mut ies = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            if data.len() - offset < 4 {
                break;
            }
            let ie = Ie::unmarshal(&data[offset..])?;
            offset += ie.len() as usize;
            ies.push(ie);
        }
        Ok(Self { ies })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new_grouped(IeType::CreateMar, self.ies.clone())
    }
}

// Builder pattern for CreateMar
pub struct CreateMarBuilder {
    mar_id: Ie,
    steering_functionality: Ie,
    steering_mode: Ie,
    tgpp_access_forwarding: Option<Ie>,
    non_tgpp_access_forwarding: Option<Ie>,
}

impl CreateMarBuilder {
    pub fn new(mar_id: Ie, steering_functionality: Ie, steering_mode: Ie) -> Self {
        Self {
            mar_id,
            steering_functionality,
            steering_mode,
            tgpp_access_forwarding: None,
            non_tgpp_access_forwarding: None,
        }
    }

    pub fn tgpp_access_forwarding(mut self, ie: Ie) -> Self {
        self.tgpp_access_forwarding = Some(ie);
        self
    }

    pub fn non_tgpp_access_forwarding(mut self, ie: Ie) -> Self {
        self.non_tgpp_access_forwarding = Some(ie);
        self
    }

    pub fn build(self) -> Result<Ie, PfcpError> {
        let mut ies = vec![
            self.mar_id,
            self.steering_functionality,
            self.steering_mode,
        ];
        if let Some(ie) = self.tgpp_access_forwarding {
            ies.push(ie);
        }
        if let Some(ie) = self.non_tgpp_access_forwarding {
            ies.push(ie);
        }
        Ok(Ie::new_grouped(IeType::CreateMar, ies))
    }
}
```

---

## Phase 8: SRR, Access Availability, and ATSSS IEs (Priority: MEDIUM-LOW)

**Effort:** Medium-High | **Total IEs:** 22
**Pattern:** Mix of grouped, flags, and structured IEs

### IEs in this phase:

**SRR (Session Report Request) — IEs 211-218:**

| IE Name | Type | Pattern |
|---------|------|---------|
| RemoveSrr | 211 | Grouped: SRR ID |
| CreateSrr | 212 | Grouped: SRR ID + Access Availability Control Info |
| UpdateSrr | 213 | Grouped: SRR ID + Access Availability Control Info |
| SessionReport | 214 | Grouped: SRR ID + Access Availability Report |
| AccessAvailabilityControlInformation | 216 | Grouped: Requested Access Availability Info |
| AccessAvailabilityReport | 218 | Grouped: Access Availability Info |

**ATSSS (Access Traffic Steering/Switching/Splitting) — IEs 220-231:**

| IE Name | Type | Pattern |
|---------|------|---------|
| ProvideAtsssControlInformation | 220 | Grouped |
| AtsssControlParameters | 221 | Grouped |
| MptcpControlInformation | 222 | Flags |
| AtsssLlControlInformation | 223 | Flags |
| PmfControlInformation | 224 | Flags |
| MptcpParameters | 225 | Grouped |
| AtsssLlParameters | 226 | Grouped |
| PmfParameters | 227 | Grouped |
| AtsssLlInformation | 231 | Flags |

**IP Multicast — IEs 188-191:**

| IE Name | Type | Pattern |
|---------|------|---------|
| IpMulticastAddressingInfo | 188 | Grouped |
| JoinIpMulticastInformationWithinUsageReport | 189 | Grouped |
| LeaveIpMulticastInformationWithinUsageReport | 190 | Grouped |

---

## Phase 9: TSN (Time-Sensitive Networking) and QoS Monitoring IEs (Priority: LOW)

**Effort:** Medium | **Total IEs:** 25
**Pattern:** Mix of grouped, flags, and value IEs

### IEs in this phase:

**TSN IEs (194-210):**

| IE Name | Type | Pattern |
|---------|------|---------|
| CreateBridgeInfoForTsc | 194 | Grouped/Flags |
| CreatedBridgeInfoForTsc | 195 | Grouped |
| TscManagementInformationWithinSessionModificationRequest | 199 | Grouped |
| TscManagementInformationWithinSessionModificationResponse | 200 | Grouped |
| TscManagementInformationWithinSessionReportRequest | 201 | Grouped |
| ClockDriftControlInformation | 203 | Grouped |
| ClockDriftReport | 205 | Grouped |

**QoS Monitoring IEs (237-248):**

| IE Name | Type | Pattern |
|---------|------|---------|
| GtpuPathQosReport | 239 | Grouped |
| QosInformationInGtpuPathQosReport | 240 | Grouped |
| QosMonitoringPerQosFlowControlInformation | 242 | Grouped |
| QosMonitoringReport | 247 | Grouped |
| MtedtControlInformation | 249 | Flags |

**Redundant Transmission IEs:**

| IE Name | Type | Pattern |
|---------|------|---------|
| RedundantTransmissionParameters | 255 | Grouped |
| UpdatedPdr | 256 | Grouped |
| RedundantTransmissionForwardingParameters | 270 | Grouped |

**Other:**

| IE Name | Type | Pattern |
|---------|------|---------|
| TransportDelayReporting | 271 | Grouped |
| PartialFailureInformation | 272 | Grouped |
| UeIpAddressPoolInformation | 233 | Grouped |
| ProvideRdsConfigurationInformation | 261 | Flags |
| RdsConfigurationInformation | 262 | Flags |
| QueryPacketRateStatusWithinSessionModificationRequest | 263 | Grouped |
| PacketRateStatusReportWithinSessionModificationResponse | 264 | Grouped |

---

## Phase 10: MBS, L2TP, DSCP, Traffic Parameters, MPQUIC, RTP, and Release 18 IEs (Priority: LOW)

**Effort:** High | **Total IEs:** ~55
**Pattern:** Various (many grouped IEs)

### MBS (Multicast/Broadcast) — IEs 300-314:

| IE Name | Type |
|---------|------|
| MbsSessionN4mbControlInformation | 300 |
| MbsMulticastParameters | 301 |
| AddMbsUnicastParameters | 302 |
| MbsSessionN4mbInformation | 303 |
| RemoveMbsUnicastParameters | 304 |
| MbsSessionIdentifier | 305 |
| MulticastTransportInformation | 306 |
| Mbsn4mbReqFlags | 307 |
| LocalIngressTunnel | 308 |
| MbsUnicastParametersId | 309 |
| MbsSessionN4ControlInformation | 310 |
| MbsSessionN4Information | 311 |
| Mbsn4RespFlags | 312 |

### L2TP — IEs 276-279:

| IE Name | Type |
|---------|------|
| L2tpTunnelInformation | 276 |
| L2tpSessionInformation | 277 |
| CreatedL2tpSession | 279 |

### DSCP-to-PPI — IEs 316-317:

| IE Name | Type |
|---------|------|
| DscpToPpiControlInformation | 316 |
| DscpToPpiMappingInformation | 317 |

### Traffic Parameter Measurement — IEs 323-329:

| IE Name | Type |
|---------|------|
| TrafficParameterMeasurementControlInformation | 323 |
| TrafficParameterMeasurementReport | 324 |
| TrafficParameterThreshold | 325 |
| DlPeriodicity | 326 |
| N6JitterMeasurement | 327 |
| TrafficParameterMeasurementIndication | 328 |
| UlPeriodicity | 329 |

### MPQUIC — IEs 330-333:

| IE Name | Type |
|---------|------|
| MpquicControlInformation | 330 |
| MpquicParameters | 331 |
| TransportMode | 333 |

### RTP — IEs 339-349:

| IE Name | Type |
|---------|------|
| MediaTransportProtocol | 339 |
| RtpHeaderExtensionInformation | 340 |
| RtpPayloadInformation | 341 |
| RtpHeaderExtensionType | 342 |
| RtpHeaderExtensionId | 343 |
| RtpPayloadType | 344 |
| RtpPayloadFormat | 345 |
| RtpHeaderExtensionAdditionalInformation | 349 |

### Remaining Release 18 IEs:

| IE Name | Type |
|---------|------|
| PeerUpRestartReport | 315 |
| QerIndications | 319 |
| VendorSpecificNodeReportType | 320 |
| ConfiguredTimeDomain | 321 |
| ReportingSuggestionInfo | 335 |
| MeasurementIndication | 337 |
| HplmnSNssai | 338 |
| ExtendedDlBufferingNotificationPolicy | 346 |
| MtSdtControlInformation | 347 |
| ReportingThresholds | 348 |
| N6RoutingInformation | 351 |
| UeLevelMeasurementsConfiguration | 353 |
| ReportingControlInformation | 389 |
| PfcpsrReqFlags | 161 |
| PfcpauReqFlags | 162 |
| DownlinkDataReport | 83 |
| ErrorIndicationReport | 99 |

---

## Implementation Checklist (Per IE)

For **each** new IE, follow these steps:

### 1. Create the module file
```
src/ie/<snake_case_name>.rs
```

### 2. Register in mod.rs
Add to `src/ie/mod.rs`:
```rust
pub mod <snake_case_name>;
```

### 3. Implement the struct with:
- `new()` constructor
- `marshal()` -> `Vec<u8>` or `[u8; N]`
- `unmarshal(data: &[u8]) -> Result<Self, PfcpError>`
- `to_ie(&self) -> Ie` (or `-> Result<Ie, PfcpError>` if marshal can fail)
- Derive `Debug, Clone, PartialEq, Eq`

### 4. Write tests:
- **Round-trip test** (marshal -> unmarshal == original)
- **Empty/short buffer test** (unmarshal error)
- **Boundary values** (0, max, edge cases)
- **to_ie() test** (correct IeType)
- **Invalid value test** (for enum/flags IEs)

### 5. Verify:
```bash
cargo test ie::<module_name>
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

---

## Summary Statistics

| Phase | Category | IE Count | Effort/IE | Priority |
|-------|----------|----------|-----------|----------|
| 1 | Simple Values (u8/u16/u32/u64) | 25 | Low | HIGH |
| 2 | Flags / Single-Byte | 18 | Low | HIGH |
| 3 | Volume/Threshold with Flags | 5 | Medium | HIGH |
| 4 | Octet String / Variable-Length | 12 | Low-Med | MEDIUM |
| 5 | IP Address IEs | 8 | Medium | MEDIUM |
| 6 | Structured Multi-Field | 10 | Medium | MEDIUM |
| 7 | MAR (Multi-Access Rules) | 10 | High | MEDIUM |
| 8 | SRR, ATSSS, Multicast | 22 | Med-High | MED-LOW |
| 9 | TSN, QoS Monitoring, Redundant TX | 25 | Medium | LOW |
| 10 | MBS, L2TP, DSCP, RTP, Rel-18 | ~55 | High | LOW |
| **Total** | | **~179** | | |

### Recommended implementation order:
1. **Phase 1 + 2** first (43 IEs) — fast wins, unblocks later phases
2. **Phase 3** next (5 IEs) — completes core measurement/quota IEs
3. **Phase 4 + 5 + 6** (30 IEs) — fills remaining simple/medium IEs
4. **Phase 7** (10 IEs) — adds MAR support (requires Phase 1+2)
5. **Phase 8** (22 IEs) — SRR and ATSSS
6. **Phase 9 + 10** (80 IEs) — advanced features (TSN, MBS, RTP, etc.)
