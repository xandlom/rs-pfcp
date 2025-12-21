//! PFD Context IE.

use crate::error::PfcpError;
use crate::ie::pfd_contents::PfdContents;
use crate::ie::{Ie, IeType};

/// Represents a PFD Context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdContext {
    pub pfd_contents: Vec<PfdContents>,
}

impl PfdContext {
    /// Creates a new PFD Context.
    pub fn new(pfd_contents: Vec<PfdContents>) -> Self {
        PfdContext { pfd_contents }
    }

    /// Marshals the PFD Context into a byte vector, which is the payload of the IE.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for contents in &self.pfd_contents {
            data.extend_from_slice(&contents.to_ie().marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a PFD Context.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pfd_contents = Vec::new();
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            if ie.ie_type == IeType::PfdContents {
                pfd_contents.push(PfdContents::unmarshal(&ie.payload)?);
            }
            offset += ie.len() as usize;
        }
        Ok(PfdContext { pfd_contents })
    }

    /// Wraps the PFD Context in a PFDContext IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfdContext, self.marshal())
    }
}
