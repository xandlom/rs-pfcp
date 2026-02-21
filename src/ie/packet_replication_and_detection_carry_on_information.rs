//! Packet Replication and Detection Carry-On Information IE.
//!
//! Per 3GPP TS 29.244 Section 8.2.129, contains flags for packet
//! replication and detection carry-on.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PacketReplicationAndDetectionCarryOnInformation: u8 {
        const PRIUEAI = 1 << 0; // Bit 1: Packet Replication Indication for UE access
        const PRIN19I = 1 << 1; // Bit 2: Packet Replication Indication for N19
        const PRIN6I = 1 << 2;  // Bit 3: Packet Replication Indication for N6
        const DCARONI = 1 << 3; // Bit 4: Detection Carry-On Indication
    }
}

impl PacketReplicationAndDetectionCarryOnInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Packet Replication and Detection Carry-On Information",
                IeType::PacketReplicationAndDetectionCarryOnInformation,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::PacketReplicationAndDetectionCarryOnInformation,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all() {
        let flags = PacketReplicationAndDetectionCarryOnInformation::PRIUEAI
            | PacketReplicationAndDetectionCarryOnInformation::PRIN19I
            | PacketReplicationAndDetectionCarryOnInformation::PRIN6I
            | PacketReplicationAndDetectionCarryOnInformation::DCARONI;
        let parsed =
            PacketReplicationAndDetectionCarryOnInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_single() {
        let flags = PacketReplicationAndDetectionCarryOnInformation::DCARONI;
        let parsed =
            PacketReplicationAndDetectionCarryOnInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PacketReplicationAndDetectionCarryOnInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PacketReplicationAndDetectionCarryOnInformation::PRIUEAI
                .to_ie()
                .ie_type,
            IeType::PacketReplicationAndDetectionCarryOnInformation
        );
    }
}
