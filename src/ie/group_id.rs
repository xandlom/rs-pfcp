//! Group ID Information Element.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Group ID.
///
/// The Group ID IE contains an octet string as a global unique identifier for a group.
/// It may be encoded using the same mechanism for the NfInstanceId as specified in 3GPP TS 29.571,
/// which is typically a UUID (16 bytes), but can be any octet string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupId {
    pub value: Vec<u8>,
}

impl GroupId {
    /// Creates a new Group ID from an octet string.
    pub fn new(value: Vec<u8>) -> Self {
        GroupId { value }
    }

    /// Creates a new Group ID from a UUID (16 bytes).
    pub fn new_uuid(uuid_bytes: [u8; 16]) -> Self {
        GroupId {
            value: uuid_bytes.to_vec(),
        }
    }

    /// Creates a new Group ID from a hex string (for UUID format).
    pub fn new_from_hex(hex_str: &str) -> Result<Self, io::Error> {
        let hex_clean = hex_str.replace("-", "").replace(" ", "");
        if hex_clean.len() % 2 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Hex string must have even length",
            ));
        }

        let mut bytes = Vec::new();
        for i in (0..hex_clean.len()).step_by(2) {
            let byte_str = &hex_clean[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid hex string"))?;
            bytes.push(byte);
        }

        Ok(GroupId { value: bytes })
    }

    /// Returns the Group ID value as a byte slice.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Returns the Group ID as a hex string.
    pub fn to_hex(&self) -> String {
        self.value
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    /// Returns the Group ID as a UUID string if it's 16 bytes.
    pub fn to_uuid_string(&self) -> Option<String> {
        if self.value.len() == 16 {
            Some(format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                self.value[0], self.value[1], self.value[2], self.value[3],
                self.value[4], self.value[5],
                self.value[6], self.value[7],
                self.value[8], self.value[9],
                self.value[10], self.value[11], self.value[12], self.value[13], self.value[14], self.value[15]
            ))
        } else {
            None
        }
    }

    /// Marshals the Group ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    /// Unmarshals a Group ID from a byte slice.
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Group ID data cannot be empty",
            ));
        }

        Ok(GroupId {
            value: data.to_vec(),
        })
    }

    /// Converts to an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::GroupId, self.marshal())
    }

    /// Returns the length of the Group ID.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns true if the Group ID is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns true if this is a UUID (16 bytes).
    pub fn is_uuid(&self) -> bool {
        self.value.len() == 16
    }
}

impl From<Vec<u8>> for GroupId {
    fn from(value: Vec<u8>) -> Self {
        GroupId::new(value)
    }
}

impl From<[u8; 16]> for GroupId {
    fn from(uuid: [u8; 16]) -> Self {
        GroupId::new_uuid(uuid)
    }
}

impl From<GroupId> for Vec<u8> {
    fn from(group_id: GroupId) -> Self {
        group_id.value
    }
}

impl AsRef<[u8]> for GroupId {
    fn as_ref(&self) -> &[u8] {
        &self.value
    }
}

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(uuid_str) = self.to_uuid_string() {
            write!(f, "{}", uuid_str)
        } else {
            write!(f, "{}", self.to_hex())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_id_basic() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let group_id = GroupId::new(data.clone());
        assert_eq!(group_id.value(), &data);
        assert_eq!(group_id.len(), 4);
        assert!(!group_id.is_empty());
        assert!(!group_id.is_uuid());
    }

    #[test]
    fn test_group_id_uuid() {
        let uuid_bytes = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ];
        let group_id = GroupId::new_uuid(uuid_bytes);

        assert_eq!(group_id.len(), 16);
        assert!(group_id.is_uuid());

        let uuid_str = group_id.to_uuid_string().unwrap();
        assert_eq!(uuid_str, "123456789abcdef0-1122-3344-5566-778899aabbcc");
    }

    #[test]
    fn test_group_id_from_hex() {
        let hex_str = "123456789abcdef0";
        let group_id = GroupId::new_from_hex(hex_str).unwrap();
        assert_eq!(
            group_id.value(),
            &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]
        );

        // Test with dashes (UUID format)
        let uuid_hex = "12345678-9abc-def0-1122-334455667788";
        let group_id = GroupId::new_from_hex(uuid_hex).unwrap();
        assert_eq!(group_id.len(), 16);
        assert!(group_id.is_uuid());
    }

    #[test]
    fn test_group_id_from_hex_errors() {
        // Odd length
        let result = GroupId::new_from_hex("123");
        assert!(result.is_err());

        // Invalid hex character
        let result = GroupId::new_from_hex("12zz");
        assert!(result.is_err());
    }

    #[test]
    fn test_group_id_to_hex() {
        let data = vec![0xde, 0xad, 0xbe, 0xef];
        let group_id = GroupId::new(data);
        assert_eq!(group_id.to_hex(), "deadbeef");
    }

    #[test]
    fn test_group_id_marshal_unmarshal() {
        let original_data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let group_id = GroupId::new(original_data.clone());

        let marshaled = group_id.marshal();
        assert_eq!(marshaled, original_data);

        let unmarshaled = GroupId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, group_id);
    }

    #[test]
    fn test_group_id_unmarshal_errors() {
        // Empty data
        let result = GroupId::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_group_id_to_ie() {
        let data = vec![0xaa, 0xbb, 0xcc, 0xdd];
        let group_id = GroupId::new(data.clone());
        let ie = group_id.to_ie();

        assert_eq!(ie.ie_type, IeType::GroupId);
        assert_eq!(ie.payload, data);
    }

    #[test]
    fn test_group_id_display() {
        // UUID format
        let uuid_bytes = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ];
        let group_id = GroupId::new_uuid(uuid_bytes);
        let display_str = format!("{}", group_id);
        assert!(display_str.contains("-")); // Should be UUID format

        // Non-UUID format
        let data = vec![0x01, 0x02, 0x03];
        let group_id = GroupId::new(data);
        let display_str = format!("{}", group_id);
        assert_eq!(display_str, "010203"); // Should be hex format
    }

    #[test]
    fn test_group_id_conversions() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let group_id: GroupId = data.clone().into();
        assert_eq!(group_id.value(), &data);

        let back_to_vec: Vec<u8> = group_id.into();
        assert_eq!(back_to_vec, data);
    }

    #[test]
    fn test_group_id_round_trip() {
        let test_cases = vec![
            vec![0x01],
            vec![0x01, 0x02, 0x03, 0x04],
            vec![
                0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                0x77, 0x88,
            ], // UUID
            vec![0xff; 32], // Large ID
        ];

        for data in test_cases {
            let original = GroupId::new(data.clone());
            let marshaled = original.marshal();
            let unmarshaled = GroupId::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled);
            assert_eq!(unmarshaled.value(), &data);
        }
    }

    #[test]
    fn test_group_id_uuid_string_format() {
        let uuid_bytes = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ];
        let group_id = GroupId::new_uuid(uuid_bytes);
        let uuid_str = group_id.to_uuid_string().unwrap();

        // Check correct UUID format: 8-4-4-4-12
        let parts: Vec<&str> = uuid_str.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }
}
