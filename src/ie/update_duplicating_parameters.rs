//! Update Duplicating Parameters IE - Modify traffic duplication settings.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Update Duplicating Parameters - Modify traffic duplication settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDuplicatingParameters {
    pub destination_interface: u8,
    pub outer_header_creation: Option<Vec<u8>>,
}

impl UpdateDuplicatingParameters {
    pub fn new(destination_interface: u8) -> Self {
        Self {
            destination_interface,
            outer_header_creation: None,
        }
    }

    pub fn with_outer_header_creation(mut self, ohc: Vec<u8>) -> Self {
        self.outer_header_creation = Some(ohc);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.destination_interface);
        if let Some(ref ohc) = self.outer_header_creation {
            buf.extend_from_slice(ohc);
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Update Duplicating Parameters",
                IeType::UpdateDuplicatingParameters,
                1,
                0,
            ));
        }

        let destination_interface = data[0];
        let outer_header_creation = if data.len() > 1 {
            Some(data[1..].to_vec())
        } else {
            None
        };

        Ok(Self {
            destination_interface,
            outer_header_creation,
        })
    }
}

impl From<UpdateDuplicatingParameters> for Ie {
    fn from(params: UpdateDuplicatingParameters) -> Self {
        Ie::new(IeType::UpdateDuplicatingParameters, params.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_duplicating_parameters_marshal_unmarshal() {
        let params = UpdateDuplicatingParameters::new(1)
            .with_outer_header_creation(vec![0x01, 0x02, 0x03]);
        let marshaled = params.marshal();
        let unmarshaled = UpdateDuplicatingParameters::unmarshal(&marshaled).unwrap();
        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_update_duplicating_parameters_minimal() {
        let params = UpdateDuplicatingParameters::new(2);
        let marshaled = params.marshal();
        let unmarshaled = UpdateDuplicatingParameters::unmarshal(&marshaled).unwrap();
        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_update_duplicating_parameters_to_ie() {
        let params = UpdateDuplicatingParameters::new(3);
        let ie: Ie = params.into();
        assert_eq!(ie.ie_type, IeType::UpdateDuplicatingParameters);
    }
}
