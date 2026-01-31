//! Query URR IE - Request immediate usage reports from specific URRs.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Query URR - Request immediate usage reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryUrr {
    pub urr_id: u32,
}

impl QueryUrr {
    pub fn new(urr_id: u32) -> Self {
        Self { urr_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.urr_id.to_be_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Query URR",
                IeType::QueryUrr,
                4,
                data.len(),
            ));
        }

        let urr_id = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(Self::new(urr_id))
    }
}

impl From<QueryUrr> for Ie {
    fn from(query_urr: QueryUrr) -> Self {
        Ie::new(IeType::QueryUrr, query_urr.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_urr_marshal_unmarshal() {
        let query_urr = QueryUrr::new(0x12345678);
        let marshaled = query_urr.marshal();
        let unmarshaled = QueryUrr::unmarshal(&marshaled).unwrap();
        assert_eq!(query_urr, unmarshaled);
    }

    #[test]
    fn test_query_urr_to_ie() {
        let query_urr = QueryUrr::new(42);
        let ie: Ie = query_urr.into();
        assert_eq!(ie.ie_type, IeType::QueryUrr);
    }

    #[test]
    fn test_query_urr_unmarshal_short() {
        let result = QueryUrr::unmarshal(&[0x12, 0x34]);
        assert!(result.is_err());
    }
}
