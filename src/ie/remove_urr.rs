use crate::ie::{Ie, IeType, UrrId};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveUrr {
    pub urr_id: UrrId,
}

impl RemoveUrr {
    pub fn new(urr_id: UrrId) -> Self {
        RemoveUrr { urr_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.urr_id.marshal().to_vec()
    }

    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemoveUrr, self.marshal())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(RemoveUrr {
            urr_id: UrrId::unmarshal(data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_urr_serialization() {
        let urr = RemoveUrr::new(UrrId::new(0x12345678));
        let marshaled = urr.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);
        
        let unmarshaled = RemoveUrr::unmarshal(&marshaled).unwrap();
        assert_eq!(urr, unmarshaled);
    }

    #[test]
    fn remove_urr_to_ie() {
        let ie = RemoveUrr::new(UrrId::new(1234)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveUrr);
        assert_eq!(ie.payload.len(), 4);
    }

    #[test]
    fn invalid_unmarshal() {
        assert!(RemoveUrr::unmarshal(&[0x00]).is_err());
    }
}
