use crate::ie::far_id::FarId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveFar {
    pub far_id: FarId,
}

impl RemoveFar {
    pub fn new(far_id: FarId) -> Self {
        RemoveFar { far_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.far_id.marshal().to_vec()
    }

    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemoveFar, self.marshal())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(RemoveFar {
            far_id: FarId::unmarshal(data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_far_serialization() {
        let far = RemoveFar::new(FarId::new(0x12345678));
        let marshaled = far.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = RemoveFar::unmarshal(&marshaled).unwrap();
        assert_eq!(far, unmarshaled);
    }

    #[test]
    fn remove_far_to_ie() {
        let ie = RemoveFar::new(FarId::new(1234)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveFar);
        assert_eq!(ie.payload.len(), 4);
    }

    #[test]
    fn invalid_unmarshal() {
        assert!(RemoveFar::unmarshal(&[0x00]).is_err());
    }
}
