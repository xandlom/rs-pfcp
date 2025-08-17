use crate::ie::qer_id::QerId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveQer {
    pub qer_id: QerId,
}

impl RemoveQer {
    pub fn new(qer_id: QerId) -> Self {
        RemoveQer { qer_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.qer_id.marshal().to_vec()
    }

    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemoveQer, self.marshal())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(RemoveQer {
            qer_id: QerId::unmarshal(data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_qer_serialization() {
        let qer = RemoveQer::new(QerId::new(0x12345678));
        let marshaled = qer.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = RemoveQer::unmarshal(&marshaled).unwrap();
        assert_eq!(qer, unmarshaled);
    }

    #[test]
    fn remove_qer_to_ie() {
        let ie = RemoveQer::new(QerId::new(1234)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveQer);
        assert_eq!(ie.payload.len(), 4);
    }

    #[test]
    fn invalid_unmarshal() {
        assert!(RemoveQer::unmarshal(&[0x00]).is_err());
    }
}
