//! NF Instance ID Information Element.
//!
//! Per 3GPP TS 29.244, contains a 16-byte UUID identifying an NF instance.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let mut uuid = [0u8; 16];
        uuid.copy_from_slice(&data[0..16]);
        Ok(Self { uuid })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NfInstanceId, self.marshal().to_vec())
    }
}

impl std::fmt::Display for NfInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format as UUID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.uuid[0], self.uuid[1], self.uuid[2], self.uuid[3],
            self.uuid[4], self.uuid[5],
            self.uuid[6], self.uuid[7],
            self.uuid[8], self.uuid[9],
            self.uuid[10], self.uuid[11], self.uuid[12], self.uuid[13], self.uuid[14], self.uuid[15],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let uuid = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let id = NfInstanceId::new(uuid);
        let parsed = NfInstanceId::unmarshal(&id.marshal()).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            NfInstanceId::unmarshal(&[0x01, 0x02]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_display() {
        let uuid = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        let id = NfInstanceId::new(uuid);
        assert_eq!(format!("{}", id), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            NfInstanceId::new([0; 16]).to_ie().ie_type,
            IeType::NfInstanceId
        );
    }
}
