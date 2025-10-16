use crate::ie::pdr_id::PdrId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovePdr {
    pub pdr_id: PdrId,
}

impl RemovePdr {
    pub fn new(pdr_id: PdrId) -> Self {
        RemovePdr { pdr_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.pdr_id.marshal().to_vec()
    }

    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemovePdr, self.marshal())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(RemovePdr {
            pdr_id: PdrId::unmarshal(data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_pdr_serialization() {
        let pdr = RemovePdr::new(PdrId::new(0x1234));
        let marshaled = pdr.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34]);

        let unmarshaled = RemovePdr::unmarshal(&marshaled).unwrap();
        assert_eq!(pdr, unmarshaled);
    }

    #[test]
    fn remove_pdr_to_ie() {
        let ie = RemovePdr::new(PdrId::new(1234)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemovePdr);
        assert_eq!(ie.payload.len(), 2);
    }

    #[test]
    fn invalid_unmarshal() {
        // Empty payload
        assert!(RemovePdr::unmarshal(&[]).is_err());
        // Too short (1 byte instead of 2)
        assert!(RemovePdr::unmarshal(&[0x00]).is_err());
    }
}
