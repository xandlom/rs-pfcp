//! SMF Set ID IE - SMF Set identification for high availability.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// SMF Set ID - Identifier for SMF Set in high availability scenarios.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmfSetId {
    pub id: String,
}

impl SmfSetId {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let id = String::from_utf8(data.to_vec()).map_err(|_| {
            PfcpError::invalid_value("SMF Set ID", "Invalid UTF-8 data", "Must be valid UTF-8 string")
        })?;
        Ok(Self::new(id))
    }
}

impl From<SmfSetId> for Ie {
    fn from(smf_set_id: SmfSetId) -> Self {
        Ie::new(IeType::SmfSetId, smf_set_id.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smf_set_id_marshal_unmarshal() {
        let smf_set_id = SmfSetId::new("smf-set-001".to_string());
        let marshaled = smf_set_id.marshal();
        let unmarshaled = SmfSetId::unmarshal(&marshaled).unwrap();
        assert_eq!(smf_set_id, unmarshaled);
    }

    #[test]
    fn test_smf_set_id_to_ie() {
        let smf_set_id = SmfSetId::new("test".to_string());
        let ie: Ie = smf_set_id.into();
        assert_eq!(ie.ie_type, IeType::SmfSetId);
    }

    #[test]
    fn test_smf_set_id_empty() {
        let smf_set_id = SmfSetId::new("".to_string());
        let marshaled = smf_set_id.marshal();
        let unmarshaled = SmfSetId::unmarshal(&marshaled).unwrap();
        assert_eq!(smf_set_id, unmarshaled);
    }
}
