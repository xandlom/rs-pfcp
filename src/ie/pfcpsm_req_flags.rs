// src/ie/pfcpsm_req_flags.rs

//! PFCPSM Req-Flags Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpsmReqFlags: u8 {
        const DROBU = 1 << 0; // Bit 1: Drop Buffered Packets
        const SNDEM = 1 << 1; // Bit 2: Send End Marker
        const QAURR = 1 << 2; // Bit 3: Query All URRs
        const ISRSI = 1 << 3; // Bit 4: Inform SMF about successful retransmission
    }
}

impl PfcpsmReqFlags {
    pub fn new(flags: u8) -> Self {
        PfcpsmReqFlags::from_bits_truncate(flags)
    }

    pub fn marshal(&self) -> [u8; 1] {
        self.bits().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPSM Req-Flags",
                IeType::PfcpsmReqFlags,
                1,
                0,
            ));
        }
        Ok(PfcpsmReqFlags::from_bits_truncate(data[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcpsm_req_flags_marshal_unmarshal() {
        let flags = PfcpsmReqFlags::DROBU | PfcpsmReqFlags::QAURR;
        let marshaled = flags.marshal();
        let unmarshaled = PfcpsmReqFlags::unmarshal(&marshaled).unwrap();
        assert_eq!(flags, unmarshaled);
    }

    #[test]
    fn test_pfcpsm_req_flags_unmarshal_invalid_data() {
        let data = [];
        let result = PfcpsmReqFlags::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        if let PfcpError::InvalidLength {
            ie_name,
            ie_type,
            expected,
            actual,
        } = err
        {
            assert_eq!(ie_name, "PFCPSM Req-Flags");
            assert_eq!(ie_type, IeType::PfcpsmReqFlags);
            assert_eq!(expected, 1);
            assert_eq!(actual, 0);
        }
    }
}
