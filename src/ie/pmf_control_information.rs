//! PMF Control Information IE (IE Type 224).
//!
//! Per 3GPP TS 29.244 Section 8.2.156, provides details of required PMF functionality.
//! Contains flags and an optional list of QFI values (present when PQPM flag is set).

use crate::error::PfcpError;
use crate::ie::qfi::Qfi;
use crate::ie::{Ie, IeType};

/// PMF Control Information.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.156
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmfControlInformation {
    /// PMFI: PMF functionality is required
    pub pmfi: bool,
    /// DRTTI: Disallow PMF RTT measurements
    pub drtti: bool,
    /// Per QoS flow performance measurement QFIs (non-empty sets PQPM flag)
    pub qfis: Vec<Qfi>,
}

impl PmfControlInformation {
    pub fn new() -> Self {
        Self {
            pmfi: false,
            drtti: false,
            qfis: Vec::new(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let pqpm = !self.qfis.is_empty();
        let mut flags = 0u8;
        if self.pmfi {
            flags |= 0x01;
        }
        if self.drtti {
            flags |= 0x02;
        }
        if pqpm {
            flags |= 0x04;
        }
        let mut data = vec![flags];
        if pqpm {
            data.push(self.qfis.len() as u8);
            for qfi in &self.qfis {
                data.push(qfi.value());
            }
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PMF Control Information",
                IeType::PmfControlInformation,
                1,
                0,
            ));
        }
        let flags = data[0];
        let pmfi = (flags & 0x01) != 0;
        let drtti = (flags & 0x02) != 0;
        let pqpm = (flags & 0x04) != 0;

        let mut qfis = Vec::new();
        if pqpm {
            if data.len() < 2 {
                return Err(PfcpError::invalid_length(
                    "PMF Control Information (QFI count)",
                    IeType::PmfControlInformation,
                    2,
                    data.len(),
                ));
            }
            let count = data[1] as usize;
            if data.len() < 2 + count {
                return Err(PfcpError::invalid_length(
                    "PMF Control Information (QFI values)",
                    IeType::PmfControlInformation,
                    2 + count,
                    data.len(),
                ));
            }
            for &byte in &data[2..2 + count] {
                qfis.push(Qfi::new(byte & 0x3F)?);
            }
        }

        Ok(Self { pmfi, drtti, qfis })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PmfControlInformation, self.marshal())
    }
}

impl Default for PmfControlInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_empty_flags() {
        let original = PmfControlInformation::new();
        let parsed = PmfControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_pmfi_drtti() {
        let original = PmfControlInformation {
            pmfi: true,
            drtti: true,
            qfis: vec![],
        };
        let parsed = PmfControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(original.marshal(), vec![0x03]);
    }

    #[test]
    fn test_marshal_unmarshal_with_qfis() {
        let original = PmfControlInformation {
            pmfi: true,
            drtti: false,
            qfis: vec![Qfi::of(5), Qfi::of(9), Qfi::of(15)],
        };
        let parsed = PmfControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
        // flags byte: PMFI(1) | PQPM(4) = 0x05; count=3; values=5,9,15
        assert_eq!(original.marshal(), vec![0x05, 3, 5, 9, 15]);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            PmfControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PmfControlInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::PmfControlInformation);
    }
}
