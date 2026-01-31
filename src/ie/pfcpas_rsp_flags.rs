//! PFCPASRsp-Flags IE - Association Setup Response flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// PFCPASRsp-Flags - Flags for Association Setup Response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfcpasRspFlags {
    pub flags: u8,
}

impl PfcpasRspFlags {
    pub fn new(flags: u8) -> Self {
        Self { flags }
    }

    /// Session Retained (PSREI) flag
    pub fn with_session_retained(mut self) -> Self {
        self.flags |= 0x01;
        self
    }

    /// IP-UP Selection (UUPSI) flag  
    pub fn with_ip_up_selection(mut self) -> Self {
        self.flags |= 0x02;
        self
    }

    pub fn has_session_retained(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_ip_up_selection(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.flags]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPASRsp-Flags",
                IeType::PfcpasRspFlags,
                1,
                0,
            ));
        }

        Ok(Self::new(data[0]))
    }
}

impl From<PfcpasRspFlags> for Ie {
    fn from(flags: PfcpasRspFlags) -> Self {
        Ie::new(IeType::PfcpasRspFlags, flags.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcpasr_flags_marshal_unmarshal() {
        let flags = PfcpasRspFlags::new(0x03)
            .with_session_retained()
            .with_ip_up_selection();
        let marshaled = flags.marshal();
        let unmarshaled = PfcpasRspFlags::unmarshal(&marshaled).unwrap();
        assert_eq!(flags, unmarshaled);
    }

    #[test]
    fn test_pfcpasr_flags_methods() {
        let flags = PfcpasRspFlags::new(0x01);
        assert!(flags.has_session_retained());
        assert!(!flags.has_ip_up_selection());
    }

    #[test]
    fn test_pfcpasr_flags_to_ie() {
        let flags = PfcpasRspFlags::new(0x02);
        let ie: Ie = flags.into();
        assert_eq!(ie.ie_type, IeType::PfcpasRspFlags);
    }
}
