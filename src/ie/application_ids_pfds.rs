//! Application IDs PFDs IE.

use crate::error::PfcpError;
use crate::ie::{application_id::ApplicationId, pfd_context::PfdContext, Ie, IeType};

/// Represents Application IDs PFDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationIdsPfds {
    pub application_id: ApplicationId,
    pub pfd_context: PfdContext,
}

impl ApplicationIdsPfds {
    /// Creates a new Application IDs PFDs.
    pub fn new(application_id: ApplicationId, pfd_context: PfdContext) -> Self {
        ApplicationIdsPfds {
            application_id,
            pfd_context,
        }
    }

    /// Marshals the Application IDs PFDs into a byte vector, which is the payload of the IE.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.application_id.to_ie().marshal());
        data.extend_from_slice(&self.pfd_context.to_ie().marshal());
        data
    }

    /// Unmarshals a byte slice into Application IDs PFDs.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut application_id = None;
        let mut pfd_context = None;
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::ApplicationId => {
                    application_id = Some(ApplicationId::unmarshal(&ie.payload)?)
                }
                IeType::PfdContext => pfd_context = Some(PfdContext::unmarshal(&ie.payload)?),
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(ApplicationIdsPfds {
            application_id: application_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::ApplicationId, IeType::ApplicationIdsPfds)
            })?,
            pfd_context: pfd_context.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::PfdContext, IeType::ApplicationIdsPfds)
            })?,
        })
    }

    /// Wraps the Application IDs PFDs in a ApplicationIDsPFDs IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ApplicationIdsPfds, self.marshal())
    }
}
